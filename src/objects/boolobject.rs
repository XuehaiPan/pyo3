use ffi;
use python::Python;
use err::PyResult;
use super::PyObject;
use conversion::{ExtractPyObject, ToPyObject};

/// Represents a Python `bool`.
pub struct PyBool(PyObject);

pyobject_newtype!(PyBool, PyBool_Check, PyBool_Type);

impl PyBool {
    /// Depending on `val`, returns `py.True()` or `py.False()`.
    #[inline]
    pub fn get(py: Python, val: bool) -> PyBool {
        if val { py.True() } else { py.False() }
    }

    /// Gets whether this boolean is `true`.
    #[inline]
    pub fn is_true(&self) -> bool {
        self.0.as_ptr() == unsafe { ::ffi::Py_True() }
    }
}

/// Converts a rust `bool` to a Python `bool`.
impl ToPyObject for bool {
    type ObjectType = PyBool;

    #[inline]
    fn to_py_object(&self, py: Python) -> PyBool {
        PyBool::get(py, *self)
    }

    #[inline]
    fn with_borrowed_ptr<F, R>(&self, _py: Python, f: F) -> R
        where F: FnOnce(*mut ffi::PyObject) -> R
    {
        // Avoid unnecessary Py_INCREF/Py_DECREF pair
        f(unsafe { if *self { ffi::Py_True() } else { ffi::Py_False() } })
    }
}

/// Converts a Python `bool` to a rust `bool`.
///
/// Fails with `TypeError` if the input is not a Python `bool`.
extract!(obj to bool; py => {
    Ok(try!(obj.cast_as::<PyBool>(py)).is_true())
});

