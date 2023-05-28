use std::ffi::CStr;
use std::ptr::null_mut;
use std::os::raw::c_char;
use winapi::ctypes::c_void;
use winapi::shared::ntdef::LPCWSTR;
use winapi::um::winuser::RT_RCDATA;
use winapi::shared::minwindef::HRSRC;
use winapi::um::winuser::MAKEINTRESOURCEW;
use winapi::um::libloaderapi::{FindResourceW, LoadResource, LockResource};

// Define struct Details in the C representation style. This is done to match the memory layout of C struct
// Matches the builder struct
#[repr(C)]
struct Details {
    fname: [c_char; 33], // first name field with max 33 characters
    lname: [c_char; 33], // last name field with max 33 characters
}

// Define the location to the resource we want to access
const STORELOCATION: u16 = 193;

// Find the resource in memory. Returns a handle to the resource.
fn find_resource() -> Result<HRSRC, &'static str> {
    let resource = unsafe { FindResourceW(null_mut(), MAKEINTRESOURCEW(STORELOCATION), RT_RCDATA as LPCWSTR) };
    if resource.is_null() {
        Err("Resource not found")
    } else {
        Ok(resource)
    }
}

// Load the resource into memory. Returns a pointer to the loaded resource.
fn load_resource(resource: HRSRC) -> Result<*mut c_void, &'static str> {
    let res_data_handle = unsafe { LoadResource(null_mut(), resource) };
    if res_data_handle.is_null() {
        Err("Failed to load resource")
    } else {
        Ok(res_data_handle)
    }
}

// Lock the resource in memory. Returns a pointer to the locked resource.
fn lock_resource(res_data_handle: *mut c_void) -> Result<*mut Details, &'static str> {
    let resource_pointer = unsafe { LockResource(res_data_handle) };
    if resource_pointer.is_null() {
        Err("Failed to lock resource")
    } else {
        Ok(resource_pointer as *mut Details)
    }
}

fn main() {
    // Find, load and lock the resource
    let resource = find_resource().expect("Error finding resource");
    let res_data_handle = load_resource(resource).expect("Error loading resource");
    let d = lock_resource(res_data_handle).expect("Error locking resource");

    // Print the first name and last name found in the locked resource (what the user entered in the builder)
    println!(" - First Name: {}", unsafe { CStr::from_ptr((*d).fname.as_ptr()) }.to_str().unwrap());
    println!(" - Last Name: {}", unsafe { CStr::from_ptr((*d).lname.as_ptr()) }.to_str().unwrap());
}
