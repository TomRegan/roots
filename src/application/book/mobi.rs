extern crate c_string;
extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;
use std::io;
use std::path::Path;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::os::raw::c_char;

type MobiData = *const mobi_sys::MOBIData;

pub struct Mobi {
    data: MobiData,
}

impl Mobi {
    pub fn new(path: &Path) -> Result<Mobi, io::Error> {
        read_file(path).map(|data| Mobi { data })
    }

    fn apply_to_str<'a>(
        &self,
        f: unsafe extern "C" fn(MobiData) -> *mut c_char,
    ) -> Option<&'a str> {
        Some(unsafe { f(self.data) })
            .filter(|ptr| !ptr.is_null())
            .map(|ptr| unsafe { CStr::from_ptr(ptr) })
            .map(|cstr| cstr.to_str())
            .and_then(|res| res.ok())
    }

    fn apply_to_string(&self, f: unsafe extern "C" fn(MobiData) -> *mut c_char) -> Option<String> {
        self.apply_to_str(f).map(|str| str.to_string())
    }

    pub fn get_author(&self) -> Option<Vec<String>> {
        self.apply_to_str(mobi_sys::mobi_meta_get_author)
            .map(|str| str.split(',').map(|str| str.trim()).collect::<Vec<_>>())
            .map(|vec| vec.iter().map(|e| e.to_string()).collect())
            .filter(|vec: &Vec<String>| !vec.is_empty())
    }

    pub fn get_title(&self) -> Option<String> {
        self.apply_to_string(mobi_sys::mobi_meta_get_title)
    }

    pub fn get_publisher(&self) -> Option<String> {
        self.apply_to_string(mobi_sys::mobi_meta_get_publisher)
    }

    pub fn get_publish_date(&self) -> Option<DateTime<Utc>> {
        self.apply_to_str(mobi_sys::mobi_meta_get_publishdate)
            .map(|date| NaiveDate::parse_from_str(date, "%Y-%m-%d"))
            .and_then(|date| date.ok())
            .map(|date| {
                let time = NaiveTime::from_hms(0, 0, 0);
                NaiveDateTime::new(date, time)
            })
            .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc))
    }

    pub fn get_imprint(&self) -> Option<String> {
        self.apply_to_string(mobi_sys::mobi_meta_get_imprint)
    }

    pub fn get_description(&self) -> Option<String> {
        self.apply_to_string(mobi_sys::mobi_meta_get_description)
    }

    pub fn get_subject(&self) -> Option<Vec<String>> {
        self.apply_to_str(mobi_sys::mobi_meta_get_subject)
            .map(|str| str.split(';').map(|str| str.trim()).collect::<Vec<_>>())
            .map(|vec| vec.iter().map(|e| e.to_string()).collect())
            .filter(|vec: &Vec<String>| !vec.is_empty())
    }

    pub fn get_asin(&self) -> Option<String> {
        self.apply_to_string(mobi_sys::mobi_meta_get_asin)
    }

    pub fn get_isbn(&self) -> Option<String> {
        self.apply_to_string(mobi_sys::mobi_meta_get_isbn)
    }
}

trait Asi8<'a> {
    fn as_i8(&self) -> *const libc::c_schar;
}

impl Asi8<'_> for &'_ str {
    fn as_i8(&'_ self) -> *const libc::c_schar {
        CString::new(*self)
            .expect("&str to i8 conversion failed")
            .into_raw()
    }
}

fn read_file(path: &Path) -> Result<MobiData, io::Error> {
    unsafe {
        let data = mobi_sys::mobi_init();
        let file = mobi_sys::fopen(path.to_str().unwrap().as_i8(), "rb".as_i8());
        let result = mobi_sys::mobi_load_file(data, file);
        mobi_sys::fclose(file);
        if result != 0 {
            mobi_sys::mobi_free(data);
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Could not read file: {}", path.display()),
            ))
        } else {
            Ok(data)
        }
    }
}
