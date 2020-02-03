use std::ffi::{CStr, CString};
use std::io::{stdout, Write};
use std::slice;

use curl_sys;
use libc::{c_char, c_void, size_t};

pub struct Curl {
    handle: *mut curl_sys::CURL
}

impl Curl {
    pub fn new() -> Curl {
        unsafe {
            let mut ret = Curl {
                handle: curl_sys::curl_easy_init(),
            };
            ret.set_write_cb().unwrap();
            return ret
        };
    }

    pub fn url(&mut self, url: &str) -> Result<(), curl_sys::CURLcode> {
        let url = CString::new(url).unwrap();
        self.setopt_str(curl_sys::CURLOPT_URL, &url)
    }

    fn perform(&mut self) -> Result<(), curl_sys::CURLcode> {
        unsafe { self.check_rc(curl_sys::curl_easy_perform(self.handle)) }
    }

    fn setopt_str(&mut self, opt: curl_sys::CURLoption, val: &CStr) -> Result<(), curl_sys::CURLcode> {
        unsafe { self.check_rc(curl_sys::curl_easy_setopt(self.handle, opt, val.as_ptr())) }
    }

    pub fn set_write_cb(&mut self) -> Result<(), curl_sys::CURLcode> {
        let cb: curl_sys::curl_write_callback = write_callback;
        unsafe { self.check_rc(curl_sys::curl_easy_setopt(self.handle, curl_sys::CURLOPT_WRITEFUNCTION, cb)) }
    }

    fn check_rc(&mut self, rc: curl_sys::CURLcode) -> Result<(), curl_sys::CURLcode> {
        if rc == curl_sys::CURLE_OK {
            return Ok(());
        }   
        Err(rc)
    }
}

impl Drop for Curl {
    fn drop(&mut self) {
        unsafe {
            curl_sys::curl_easy_cleanup(self.handle);
        }
    }
}

extern "C" fn write_callback(
    ptr: *mut c_char,
    size: size_t,
    nmemb: size_t,
    _userdata: *mut c_void,
) -> size_t {
    unsafe {
        let data = slice::from_raw_parts(ptr as *const u8, size * nmemb);
        stdout().write_all(data).unwrap();
        data.len()
    }
}

fn main() {
    let mut curl = Curl::new();
    curl.url("https://www.rust-lang.org/").unwrap();
    curl.perform().unwrap();
}