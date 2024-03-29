use crate::errors::Errcode;
use std::ffi::CString;
use std::path::PathBuf;

#[derive(Clone)]
// These are required to perfrom an execve syscall, which we will use
// to contain our software in a process whose execution context is restricted.
pub struct ContainerOpts{
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32, // this is id of the user inside the container, ID 0 means root.
    pub mount_dir: PathBuf
}



impl ContainerOpts {
    pub fn new(command: String, uid:u32, mount_dir: PathBuf) -> Result<ContainerOpts, Errcode> {
        let argv : Vec<CString> = command.split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg")).collect();
        let path = argv[0].clone();
        Ok(
            ContainerOpts{
                path,
                argv,
                uid,
                mount_dir
            }
          )
    }
}





















