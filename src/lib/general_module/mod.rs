use libloading::Library;

use super::{QCModule, ERROR};
use std::ffi::{c_int, c_char, CString};

type FuncRun<'a> = libloading::Symbol<'a, unsafe extern "C" fn(level: c_int, datetime: *const c_char, data: *const c_char) -> c_int>;

pub struct GeneralModule {
    lib: Library,
}

impl QCModule for GeneralModule {
    fn run(&self, level: usize, datetime: &chrono::NaiveDateTime, data: &super::data_parser::DataType) -> Result<bool, super::ERROR> {
        let c_level = level as i32;
        let c_datetime = CString::new(datetime.format("%Y-%m-%dT%H:%M:%S").to_string()).unwrap();
        let c_data = CString::new(data.to_string()).unwrap();
        
        unsafe {
            let func: FuncRun = self.lib.get(b"run").expect("Cannot load fundtion");
            func(c_level, c_datetime.as_ptr(), c_data.as_ptr());
        }
        
        Ok(true)
    }
}

impl GeneralModule {
    pub fn new(path: &str) -> Result<Self, ERROR> {
        unsafe {
            let lib = libloading::Library::new(path)
                .expect("Cannot load share library");
            Ok(Self { lib })
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::{c_int, c_char, CString};

    use chrono::NaiveDateTime;

    use crate::lib::data_parser::DataType;

    use super::*;
    #[test]
    fn case1() {
        unsafe {
            let lib = libloading::Library::new("module/c/libmain.so").expect("Cannot load share library");
            let func: libloading::Symbol<unsafe extern "C" fn(level: c_int, datetime: *const c_char, data: *const c_char)> = lib.get(b"run").expect("Cannot load fundtion");
            
            let level = 3;
            let datetime = CString::new("2023-01-02T00:03:55").unwrap();
            let data = CString::new("hello from rust").unwrap();
            func(level, datetime.as_ptr(), data.as_ptr());
        }
    }

    #[test]
    fn case2() {
        let module = GeneralModule::new("module/c/libmain.so").unwrap();
        let ret = module.run(
            3, 
            &NaiveDateTime::parse_from_str("2023-01-02T00:03:55", "%Y-%m-%dT%H:%M:%S").unwrap(), 
            &DataType::Float(55.3)
        );
        println!("{:?}", ret);
    }
}