use std::{fs::File, io::Read, path::Path, collections::HashMap};

use pyo3::{prelude::*, types::{PyDict, IntoPyDict}};

struct PythonModule {
    name: String,
    src_code: String,
}

impl PythonModule {
    pub fn new<S:AsRef<Path> + Copy>(name: &str, path: S) -> Self {
        let mut file = File::open(path)
            .expect("Can't open config file.");
        let mut src_code = String::new();
        file.read_to_string(&mut src_code)
            .expect("Failed to read file.");

        Self { 
            name: name.to_string(), 
            src_code 
        }
    }

    pub fn run(&self) -> PyResult<()> {
        Python::with_gil(|py| {
            let func: Py<PyAny> = PyModule::from_code(
                py, &self.src_code, &format!("{}.py", self.name), &self.name
            )?
            .getattr("run")?
            .into();

            let mut map = HashMap::new();
            map.insert("datetime", "2023-01-03".to_object(py));
            map.insert("data", 333.to_object(py));

            let pyobj = func.call(py, (), Some(map.into_py_dict(py)))?;
            let res: &PyDict = pyobj.extract(py)?;
            println!("python return: {:?}", res.get_item("res")?);
            Ok(())
        })
    }
}


#[cfg(test)]
mod test {
    use std::thread::sleep;

    use super::*;
    #[test]
    fn case1() {
        let path = "./module/hello_world.py";
        let py = PythonModule::new("helloworld", path);

        // println!("src code: {:?}", py.src_code);
        let _ = py.run();
    }
}