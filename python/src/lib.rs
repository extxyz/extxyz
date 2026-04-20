use std::io::BufReader;

use extxyz::{read_frame as rs_read_frame, Frame, Value as InnerValue};
use pyo3::{prelude::*, types::PyDict};

#[pyclass]
#[pyo3(name = "Frame")]
struct PyFrame(Frame);

struct Value(InnerValue);

impl<'py> IntoPyObject<'py> for Value {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            InnerValue::Integer(integer) => todo!(),
            InnerValue::Float(float_num) => todo!(),
            InnerValue::Bool(boolean) => todo!(),
            InnerValue::Str(text) => todo!(),
            InnerValue::VecInteger(integers, _) => todo!(),
            InnerValue::VecFloat(float_nums, _) => todo!(),
            InnerValue::VecBool(booleans, _) => todo!(),
            InnerValue::VecText(texts, _) => todo!(),
            InnerValue::MatrixInteger(items, _) => todo!(),
            InnerValue::MatrixFloat(items, _) => todo!(),
            InnerValue::MatrixBool(items, _) => todo!(),
            InnerValue::MatrixText(items, _) => todo!(),
            InnerValue::Unsupported => todo!(),
        }
    }
}

#[pymethods]
impl PyFrame {
    fn natoms(self_: PyRef<'_, Self>) -> PyResult<u32> {
        Ok(self_.0.natoms())
    }

    fn arrs(self_: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let dict = PyDict::new(py);
        let arrs = self_.0.arrs();
        for (k, v) in arrs {
            let v = Value(v.clone());
            dict.set_item(k, v.into_pyobject(py)?)?;
        }

        Ok(dict.into())
    }

    fn info(self_: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let dict = PyDict::new(py);
        let arrs = self_.0.info();
        for (k, v) in arrs {
            let v = Value(v.clone());
            dict.set_item(k, v.into_pyobject(py)?)?;
        }

        Ok(dict.into())
    }
}

struct PyTextIO {
    // a TextIO
    obj: Py<PyAny>,
}

impl PyTextIO {
    fn new(stream: Py<PyAny>) -> Self {
        PyTextIO { obj: stream }
    }
}

impl std::io::Read for PyTextIO {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Python::attach(|py| -> PyResult<usize> {
            let obj_textio = self.obj.as_ref();
            let bytes = obj_textio
                .call_method1(py, "read", (buf.len(),))?
                .extract::<Vec<u8>>(py)?;

            let n = bytes.len();
            buf[..n].copy_from_slice(&bytes);
            Ok(n)
        })
        .map_err(|e| std::io::Error::other(e.to_string()))
    }
}

#[pyfunction]
fn py_read_frame(_py: Python, stream: Py<PyAny>) -> PyResult<PyFrame> {
    let rd = PyTextIO::new(stream);
    let mut rd = BufReader::new(rd);
    let frame = rs_read_frame(&mut rd)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    Ok(PyFrame(frame))
}

#[pymodule]
#[pyo3(name = "extxyz")]
fn pyextxyz(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFrame>()?;
    m.add_function(wrap_pyfunction!(py_read_frame, m)?)?;
    Ok(())
}
