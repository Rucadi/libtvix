
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Internal function to evaluate Nix code and return its string result
use std::env;
use tvix_eval::Evaluation;

fn evaluate_code(code: &str) -> String {
    // Build pure evaluator
    let mut out = String::new();
    let eval_builder = Evaluation::builder_impure();

    // Get current working directory
    let current_dir = env::current_dir()
        .unwrap_or_else(|_| "/".into())
        .to_string_lossy()
        .into_owned();

    // Build and run evaluation
    let evaluator = eval_builder.build();
    let result = evaluator.evaluate(code, Some(current_dir.into()));

    // Only capture the value
    if let Some(val) = result.value {
        out = val.to_string();
    }
    out
}


/// Evaluates a Nix expression string and returns a newly allocated C string pointer.
/// Caller must free the returned string via `free_cstring`.
#[no_mangle]
pub extern "C" fn eval_nix_expr(code_ptr: *const c_char) -> *mut c_char {
    if code_ptr.is_null() {
        return std::ptr::null_mut();
    }
    // Safety: expect a null-terminated C string
    let cstr = unsafe { CStr::from_ptr(code_ptr) };
    let code = match cstr.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    // Get result
    let result_string = evaluate_code(code);

    // Convert to CString and return to caller
    match CString::new(result_string) {
        Ok(cstring) => cstring.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees a C string previously returned by `eval_nix_expr`.
#[no_mangle]
pub extern "C" fn free_cstring(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { let _ = CString::from_raw(s); }
}
