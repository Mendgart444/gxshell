#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn response() {
        // Checks the version of the console.
        if GXSHELL_VERSION == "0.1.0" {
            println!("ok");
        } else {
            panic!("GXshell version is unknown!!!")
        }
        
        if GXINSTALLER_VERSION == "0.1.0" {
            println!("Released version");
        } else {
            panic!("GXinstaller Version is unknown!!!")
        }
    
    }
}

pub const  GXINSTALLER_VERSION:&'static str = "0.1.0";
pub const  GXSHELL_VERSION:&'static str = "0.1.0";

