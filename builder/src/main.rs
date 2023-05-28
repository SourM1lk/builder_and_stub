use std::ffi::OsStr;
use std::iter::once;
use std::io::{self, Write};
use winapi::ctypes::c_void;
use winapi::um::winuser::RT_RCDATA;
use winapi::shared::ntdef::LPCWSTR;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winnt::{HANDLE, MAKELANGID, LANG_NEUTRAL, SUBLANG_NEUTRAL};
use winapi::um::winbase::{BeginUpdateResourceW, EndUpdateResourceA, UpdateResourceW};

// Define struct Details in the C representation style. This is done to match the memory layout of C struct
// Matches the stub struct
#[repr(C)]
struct Details {
    fname: [i8; 33],
    lname: [i8; 33],
}

// Define the location to the resource we want to access
const STUB: &'static str = "Stub.exe";
const STUBNEW: &'static str = "Test.exe";
const STORELOCATION: u16 = 193;

fn fill_details(details: &mut Details) -> io::Result<()> {
    // Get the first and last name from the user
    let fname = input(" - Enter first name: ")?;
    let lname = input(" - Enter last name: ")?;

    // Check if the input names are too long for the details structure (33 characters)
    if fname.len() > details.fname.len() || lname.len() > details.lname.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Name is too long"));
    }

    // Convert u8 to i8 and copy the names into the details structure
    details.fname[..fname.len()].copy_from_slice(&fname.as_bytes().iter().map(|&x| x as i8).collect::<Vec<i8>>());
    details.lname[..lname.len()].copy_from_slice(&lname.as_bytes().iter().map(|&x| x as i8).collect::<Vec<i8>>());

    Ok(())
}

// Get input from the user
fn input(prompt: &str) -> io::Result<String> {
    let mut input = String::new();

    // Promt the user for input
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

fn update_resource(details: &Details) -> io::Result<()> {
    // Copy the stub to a new file (test.exe)
    println!(" - Removing existing test.exe");
    std::fs::copy(STUB, STUBNEW)?;

    
    let stubnew_w: Vec<u16> = OsStr::new(STUBNEW).encode_wide().chain(once(0)).collect();
    let details_ptr = details as *const _ as *mut c_void;

    unsafe {
        // Begin the update in the new file's resource section
        let h_update: HANDLE = BeginUpdateResourceW(stubnew_w.as_ptr(), 0);
        if h_update.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed BeginUpdateResource"));
        }

        // Update the resource with the details structure
        let res = UpdateResourceW(
            h_update,
            RT_RCDATA as LPCWSTR,
            MAKEINTRESOURCEW(STORELOCATION),
            MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL),
            details_ptr,
            std::mem::size_of::<Details>() as u32,
        );

        // Check if the update was successful
        if res == 0 {
            EndUpdateResourceA(h_update, 1);
            return Err(io::Error::new(io::ErrorKind::Other, "Failed UpdateResource"));
        }

        // End the update
        EndUpdateResourceA(h_update, 0);
    }

    Ok(())
}

// Converts a numeric resource identifier to a string resource identifier.
unsafe fn MAKEINTRESOURCEW(i: u16) -> LPCWSTR {
    i as LPCWSTR
}

fn main() {
    let mut details = Details { fname: [0; 33], lname: [0; 33] };

    // Fill the details structure with the user input
    if let Err(err) = fill_details(&mut details) {
        eprintln!("Failed to fill details: {}", err);
        return;
    }

    // Update the resource with the details structure
    match update_resource(&details) {
        Ok(_) => println!(" - Success, run test.exe to confirm."),
        Err(err) => eprintln!("Failed to update resource: {}", err),
    }
}
