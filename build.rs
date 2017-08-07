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
    gen_enums(0, 6).unwrap();
}

fn gen_enums(min: usize, max: usize) -> Result<(), Error> {
    let mod_path = "enums.rs";

    // let out_dir = env::var("OUT_DIR")?;
    let out_dir = "src";
    let out_path = Path::new(&out_dir).join(mod_path);
    let mut out = File::create(&out_path)?;

    writeln!(out, "use core::mem::transmute;")?;
    writeln!(out, "use core::fmt;")?;
    writeln!(out, "")?;

    for i in min..max {
        gen_type(&mut out, i + 1)?;
    }
    Ok(())
}

fn gen_type(out: &mut File,  size: usize) -> Result<(), Error> {
    let carrier_name = "u8";
    let type_name = format!("U{}", size);
    // println!("cargo:warning=Generating {} in {:?}", type_name, out_path);
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
        writeln!(out, "    fn from(other: {}) -> {} {{", type_name, other_type)?;
        writeln!(out, "        other as {}", other_type)?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;
        
    }
    writeln!(out, "impl PartialEq<i32> for {} {{", type_name)?;
    writeln!(out, "    fn eq(&self, other: &i32) -> bool {{")?;
    writeln!(out, "        *self as i32 == *other ")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "")?;
       
    writeln!(out, "impl {} {{", type_name)?;
    writeln!(out, "   pub fn value(&self) -> u8 {{ *self as u8 }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "")?;

    writeln!(out, "impl fmt::Debug for {} {{", type_name)?;
    writeln!(out, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(out, "       write!(f, \"0b{{:0width$b}}\", *self as u8, width={})", size)?;
    writeln!(out, "    }}")?;
    writeln!(out, " }}")?;
    writeln!(out, "")?;


    writeln!(out, "impl fmt::Display for {} {{", type_name)?;
    writeln!(out, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(out, "       (*self as u8).fmt(f)")?;
    writeln!(out, "    }}")?;
    writeln!(out, " }}")?;
    writeln!(out, "")?;

    writeln!(out, "impl fmt::LowerHex for {} {{", type_name)?;
    writeln!(out, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(out, "       (*self as u8).fmt(f)")?;
    writeln!(out, "    }}")?;
    writeln!(out, " }}")?;
    writeln!(out, "")?;

    Ok(())
}