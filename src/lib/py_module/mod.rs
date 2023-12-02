use std::{collections::HashMap, fs::File, io::Read, path::Path};

use chrono::NaiveDateTime;
use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{IntoPyDict, PyBool, PyDict},
};

use super::{data_parser::DataType, QCModule, ERROR};

pub struct PythonModule {
    name: String,
    src_code: String,
}

impl ToPyObject for DataType {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match &self {
            Self::Datetime(v) => v.to_string().to_object(py),
            Self::Float(v) => v.to_object(py),
            Self::Integer(v) => v.to_object(py),
            Self::String(v) => v.to_object(py),
            Self::NULL => py.None(),
        }
    }
}

impl QCModule for PythonModule {
    fn run(&self, level: usize, datetime: &NaiveDateTime, data: &DataType) -> Result<bool, ERROR> {
        match self._run(level, datetime, data) {
            Ok(status) => Ok(status),
            Err(v) => Err(Box::new(v)),
        }
    }
}

impl PythonModule {
    pub fn new<S: AsRef<Path> + Copy>(name: &str, path: S) -> Result<Self, ERROR> {
        let mut file = File::open(path).expect("Can't open config file.");
        let mut src_code = String::new();
        file.read_to_string(&mut src_code)
            .expect("Failed to read file.");

        Ok(Self {
            name: name.to_string(),
            src_code,
        })
    }

    fn _run(&self, level: usize, datetime: &NaiveDateTime, data: &DataType) -> PyResult<bool> {
        Python::with_gil(|py| {
            let func: Py<PyAny> =
                PyModule::from_code(py, &self.src_code, &format!("{}.py", self.name), &self.name)?
                    .getattr("run")?
                    .into();

            // let datetime = NaiveDateTime::from_str("2023-01-02T10:11:32").unwrap();
            // let data = DataType::Float(32.0);

            let mut map = HashMap::new();
            map.insert("level", level.to_object(py));
            map.insert("datetime", datetime.to_string().to_object(py));
            map.insert("data", data.to_object(py));

            let pyobj = func.call(py, (), Some(map.into_py_dict(py)))?;
            let res: &PyDict = pyobj.extract(py)?;
            // println!("python return: {:?}", res.get_item("res")?);
            if let Some(v) = res.get_item("res")? {
                let status = v.downcast_exact::<PyBool>()?;
                Ok(status.is_true())
            } else {
                PyResult::Err(PyTypeError::new_err("Missing return value"))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use std::{str::FromStr, thread::sleep};

    use super::*;
    #[test]
    fn case1() {
        let path = "./module/hello_world.py";
        let py = PythonModule::new("helloworld", path).unwrap();

        // println!("src code: {:?}", py.src_code);

        let datetime = NaiveDateTime::from_str("2023-01-02T10:11:32").unwrap();
        let data = DataType::Float(32.0);

        let result = py.run(0, &datetime, &data);
        println!("result: {:?}", result);
    }
}
