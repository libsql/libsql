use std::ffi::{c_char, c_int, c_void};
type sqlite3 = c_void; //TODO

#[link(name="sqlite3")]
extern "C" {
    fn sqlite3_open(
        filename: *const c_char, 
        ppDb: *mut*mut sqlite3,
    ) -> c_int;

    fn sqlite3_close(
        ppDb: *mut sqlite3,
    ) -> c_int;

    fn sqlite3_exec(
        ppDb: *mut sqlite3, 
        sql: *const c_char, 
        callback: unsafe extern "C" fn(*mut c_void, *mut*mut c_char, *mut*mut c_char) -> c_int,
        arg1: *mut c_void,
        errmsg: *mut*mut c_char,
    ) -> c_int;

    fn sqlite3_free(
        errmsg: *mut c_char,
    );

    fn sqlite3_errmsg(
        ppDb: *mut sqlite3,
    ) -> *const c_char;
}

//TODO: fix this:
fn callback(notUsed: *mut c_void, argc: c_int, argv: *mut*mut c_char, azColName: *mut*mut c_char) -> c_int {
    for i in 0..argc {
        println!("{} = {}", azColName[i], if argv.is_null() { "NULL" } else { azColName[i] });
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use std::ffi::{CString, CStr};

    #[test]
    fn test_select_42() {
        let db: *mut sqlite3 = std::ptr::null_mut();
        let sql = CString::new("SELECT 42").expect("CString::new failed");
        let db_name = CString::new("Hello!").expect("CString::new failed");

        let db_ptr = &mut db as *mut*mut c_void;
        let rc = unsafe { sqlite3_open(db_name.as_ptr(), db_ptr) };
        if rc != 0 {
            let errmsg = unsafe {
                let s = sqlite3_errmsg(db);
                str::from_utf8_unchecked(CStr::from_ptr(s).to_bytes())
            };
            eprintln!("Can't open database: {}", errmsg);

            let rc = unsafe { sqlite3_close(db) };
            let errmsg = unsafe {
                let s = sqlite3_errmsg(db);
                str::from_utf8_unchecked(CStr::from_ptr(s).to_bytes())
            };
            if rc != 0 {
                eprintln!("Error: sqlite3_close() returns {}: {}", rc, errmsg);
            }
        }
        assert_eq!(rc, 0);

        let s: *mut c_char = std::ptr::null_mut();
        let s_ptr = &mut s as *mut*mut c_char; 
        let rc = unsafe { sqlite3_exec(db, sql.as_ptr(), callback, std::ptr::null_mut(), s_ptr) };
        if rc != 0 {
            let errmsg = unsafe { str::from_utf8_unchecked(CStr::from_ptr(s).to_bytes()) };
            eprintln!("SQL error: {}", errmsg);
            unsafe { sqlite3_free(s) };
        }
        assert_eq!(rc, 0);

        let rc = unsafe { sqlite3_close(db) };
        if rc != 0 {
            let errmsg = unsafe {
                let s = sqlite3_errmsg(db);
                str::from_utf8_unchecked(CStr::from_ptr(s).to_bytes())
            };
            eprintln!("Error: sqlite3_close() returns {}: {}", rc, errmsg);
        }
        assert_eq!(rc, 0);
    }
}
