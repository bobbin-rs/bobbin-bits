use std::env;
use std::io::{self, Write};
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    VarError(env::VarError),
    IoError(io::Error),
}

impl From<env::VarError> for Error {
    fn from(other: env::VarError) -> Self {
        Error::VarError(other)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IoError(other)
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    gen_enums(1, 6).unwrap();
    gen_ranges(1, 32).unwrap();
}

fn gen_enums(min: usize, max: usize) -> Result<(), Error> {
    let mod_path = "enums.rs";

    // let out_dir = env::var("OUT_DIR")?;
    let out_dir = "src";
    let out_path = Path::new(&out_dir).join(mod_path);
    let mut out = File::create(&out_path)?;

    writeln!(out, "use core::mem::transmute;")?;
    writeln!(out, "use core::ops::Not;")?;
    writeln!(out, "use core::fmt;")?;
    writeln!(out, "")?;

    for i in min..max+1 {
        gen_enum_type(&mut out, i)?;
    }


    writeln!(out, "impl From<bool> for U1 {{")?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    fn from(other: bool) -> U1 {{")?;
    writeln!(out, "        U1::from(other as u8)")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "")?;

    writeln!(out, "impl U1 {{")?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub fn is_set(&self) -> bool {{")?;
    writeln!(out, "        *self as u8 != 0")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "")?;

    writeln!(out, "impl Not for U1 {{")?;
    writeln!(out, "    type Output = U1;")?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    fn not(self) -> U1 {{")?;
    writeln!(out, "        U1::from(self as u8)")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;    
    Ok(())
}

fn gen_enum_type(out: &mut File,  size: usize) -> Result<(), Error> {
    let carrier_name = "u8";
    let type_name = format!("U{}", size);
    writeln!(out, "#[derive(PartialEq, Eq, Clone, Copy)]")?;
    writeln!(out, "#[repr({})]", carrier_name)?;
    writeln!(out, "pub enum {} {{", type_name)?;
    for i in 0..(1 << size) {
        writeln!(out, "    B{:0width$b} = 0b{:0width$b},", i, i, width=size)?;
    }
    writeln!(out, "}}")?;
    writeln!(out, "")?;

    for other_type in &["u8", "u16", "u32", "usize", "i32"] {

        writeln!(out, "impl From<{}> for {} {{", other_type, type_name)?;
        writeln!(out, "    #[inline]")?;
        writeln!(out, "    fn from(other: {}) -> Self {{", other_type)?;
        if other_type == &"i32" {
            writeln!(out, "        assert!(other >= 0);")?;            
        }
        writeln!(out, "        assert!(other & !0b{:b} == 0);", (1 << size) - 1)?;
        writeln!(out, "        unsafe {{ transmute(other as {}) }}", carrier_name)?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;

        writeln!(out, "impl From<{}> for {} {{", type_name, other_type)?;
        writeln!(out, "    #[inline]")?;
        writeln!(out, "    fn from(other: {}) -> {} {{", type_name, other_type)?;
        writeln!(out, "        other as {}", other_type)?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;
        
    }
    writeln!(out, "impl PartialEq<i32> for {} {{", type_name)?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    fn eq(&self, other: &i32) -> bool {{")?;
    writeln!(out, "        *self as i32 == *other ")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "")?;
       
    writeln!(out, "impl {} {{", type_name)?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub fn value(&self) -> {} {{", carrier_name)?;
    writeln!(out, "        *self as {}", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;    
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub unsafe fn from_u8_unchecked(other: u8) -> Self {{")?;
    writeln!(out, "        transmute(other as {})", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub unsafe fn from_u16_unchecked(other: u16) -> Self {{")?;
    writeln!(out, "        transmute(other as {})", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub unsafe fn from_u32_unchecked(other: u32) -> Self {{")?;
    writeln!(out, "        transmute(other as {})", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub unsafe fn from_usize_unchecked(other: usize) -> Self {{")?;
    writeln!(out, "        transmute(other as {})", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub fn into_u8(self) -> u8 {{")?;
    writeln!(out, "        self as u8")?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;    
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub fn into_u16(self) -> u16 {{")?;
    writeln!(out, "        self as u16")?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;    
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub fn into_u32(self) -> u32 {{")?;
    writeln!(out, "        self as u32")?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;    
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub fn into_usize(self) -> usize {{")?;
    writeln!(out, "        self as usize")?;
    writeln!(out, "    }}")?;    
    writeln!(out, "")?;    
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub fn into_i32(self) -> i32 {{")?;
    writeln!(out, "        self as i32")?;
    writeln!(out, "    }}")?;    
    writeln!(out, "}}")?;    
    writeln!(out, "")?;

    writeln!(out, "impl fmt::Debug for {} {{", type_name)?;
    writeln!(out, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(out, "        write!(f, \"0b{{:0width$b}}\", *self as u8, width={})", size)?;
    writeln!(out, "    }}")?;
    writeln!(out, " }}")?;
    writeln!(out, "")?;


    writeln!(out, "impl fmt::Display for {} {{", type_name)?;
    writeln!(out, "    #[inline]")?;    
    writeln!(out, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(out, "        (*self as u8).fmt(f)")?;
    writeln!(out, "    }}")?;
    writeln!(out, " }}")?;
    writeln!(out, "")?;

    writeln!(out, "impl fmt::LowerHex for {} {{", type_name)?;
    writeln!(out, "    #[inline]")?;    
    writeln!(out, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(out, "        (*self as u8).fmt(f)")?;
    writeln!(out, "    }}")?;
    writeln!(out, " }}")?;
    writeln!(out, "")?;

    Ok(())
}


fn gen_ranges(min: usize, max: usize) -> Result<(), Error> {
    let mod_path = "ranges.rs";

    let out_dir = "src";
    let out_path = Path::new(&out_dir).join(mod_path);
    let mut out = File::create(&out_path)?;

    writeln!(out, "use core::mem::transmute;")?;
    writeln!(out, "use core::fmt;")?;
    writeln!(out, "")?;

    for i in min..max+1 {
        gen_range_type(&mut out, i)?;
    }
    Ok(())
}

fn gen_range_type(out: &mut File, size: usize) -> Result<(), Error> {
    let carrier_name = "usize";
    let type_name = format!("R{}", size);
    let width = if size <= 16 { 1 } else { 2 };    
    writeln!(out, "#[derive(PartialEq, Eq, Clone, Copy)]")?;
    writeln!(out, "#[repr({})]", carrier_name)?;
    writeln!(out, "pub enum {} {{", type_name)?;
    for i in 0..size {        
        writeln!(out, "    X{:0width$x} = 0x{:0width$x},", i, i, width=width)?;
    }
    writeln!(out, "}}")?;
    writeln!(out, "")?;

    for i in 2..size {
        let other_type = format!("R{}", i);
        writeln!(out, "impl From<{}> for {} {{", other_type, type_name)?;
        writeln!(out, "    #[inline]")?;
        writeln!(out, "    fn from(other: {}) -> Self {{", other_type)?;
        writeln!(out, "        unsafe {{ transmute(other as {}) }}", carrier_name)?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;        
    }

    for other_type in &["u8", "u16", "u32", "usize", "i32"] {
        writeln!(out, "impl From<{}> for {} {{", other_type, type_name)?;
        writeln!(out, "    #[inline]")?;
        writeln!(out, "    fn from(other: {}) -> Self {{", other_type)?;
        if other_type == &"i32" {
            writeln!(out, "        assert!(other >= 0);")?;            
        }
        writeln!(out, "        assert!(other < 0x{:x});", size)?;
        writeln!(out, "        unsafe {{ transmute(other as {}) }}", carrier_name)?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;

        writeln!(out, "impl From<{}> for {} {{", type_name, other_type)?;
        writeln!(out, "    #[inline]")?;
        writeln!(out, "    fn from(other: {}) -> {} {{", type_name, other_type)?;
        writeln!(out, "        other as {}", other_type)?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;
        
        writeln!(out, "impl PartialEq<{}> for {} {{", other_type, type_name)?;
        writeln!(out, "    #[inline]")?;
        writeln!(out, "    fn eq(&self, other: &{}) -> bool {{", other_type)?;
        writeln!(out, "        *self as usize == *other as usize")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;
    }

       
    writeln!(out, "impl {} {{", type_name)?;
    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub fn value(&self) -> {} {{", carrier_name)?;
    writeln!(out, "        *self as {}", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;    

    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub unsafe fn from_u8_unchecked(other: u8) -> Self {{")?;
    writeln!(out, "        transmute(other as {})", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;

    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub unsafe fn from_u16_unchecked(other: u16) -> Self {{")?;
    writeln!(out, "        transmute(other as {})", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;

    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub unsafe fn from_u32_unchecked(other: u32) -> Self {{")?;
    writeln!(out, "        transmute(other as {})", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;

    writeln!(out, "    #[inline]")?;
    writeln!(out, "    pub unsafe fn from_usize_unchecked(other: usize) -> Self {{")?;
    writeln!(out, "        transmute(other as {})", carrier_name)?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "")?;

    writeln!(out, "impl fmt::Debug for {} {{", type_name)?;
    writeln!(out, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(out, "        write!(f, \"0x{{:0{}x}}\", *self as u8)", width)?;
    writeln!(out, "    }}")?;
    writeln!(out, " }}")?;
    writeln!(out, "")?;


    writeln!(out, "impl fmt::Display for {} {{", type_name)?;
    writeln!(out, "    #[inline]")?;    
    writeln!(out, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(out, "       (*self as u8).fmt(f)")?;
    writeln!(out, "    }}")?;
    writeln!(out, " }}")?;
    writeln!(out, "")?;

    writeln!(out, "impl fmt::LowerHex for {} {{", type_name)?;
    writeln!(out, "    #[inline]")?;    
    writeln!(out, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(out, "       (*self as u8).fmt(f)")?;
    writeln!(out, "    }}")?;
    writeln!(out, " }}")?;
    writeln!(out, "")?;
    Ok(())
}
