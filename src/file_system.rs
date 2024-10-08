use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::fs::metadata;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
#[cfg(target_family = "unix")]
use std::os::unix::fs::symlink;


#[cfg(target_family = "windows")]
use std::os::windows::fs::{symlink_file, symlink_dir};

pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    let original = original.as_ref();
    let link = link.as_ref();

    #[cfg(target_family = "unix")]
    {
        std::os::unix::fs::symlink(original, link)
    }

    #[cfg(target_family = "windows")]
    {
        if original.is_dir() {
            symlink_dir(original, link)
        } else {
            symlink_file(original, link)
        }
    }
}

#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_family = "unix")]
fn get_owner_unix(path: &Path) -> io::Result<OsString> {
    let metadata = metadata(path)?;
    let uid = metadata.uid();
    // Use 'nix' or 'libc' to get the owner name from the UID
    let pw = unsafe { libc::getpwuid(uid) };
    if pw.is_null() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Owner not found"));
    }
    let owner = unsafe { std::ffi::CStr::from_ptr((*pw).pw_name) }.to_owned();
    Ok(owner.into())
}

#[cfg(target_family = "windows")]
fn get_owner_windows(path: &Path) -> io::Result<OsString> {
    use std::os::windows::ffi::OsStringExt;
    use winapi::um::accctrl::SE_FILE_OBJECT;
    use winapi::um::aclapi::GetNamedSecurityInfoW;
    use winapi::um::winnt::{PSID, OWNER_SECURITY_INFORMATION};
    use winapi::shared::winerror::ERROR_SUCCESS;
    use winapi::shared::sddl::ConvertSidToStringSidW;
    use std::ptr;

    let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let mut owner_sid: PSID = ptr::null_mut();
    let mut sec_desc: *mut winapi::ctypes::c_void = ptr::null_mut();

    let result = unsafe {
        GetNamedSecurityInfoW(
            path_wide.as_ptr(),
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION,
            &mut owner_sid,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut sec_desc,
        )
    };

    if result != ERROR_SUCCESS {
        return Err(io::Error::last_os_error());
    }

    let mut owner_str: *mut u16 = ptr::null_mut();
    let success = unsafe { ConvertSidToStringSidW(owner_sid, &mut owner_str) };

    if success == 0 {
        return Err(io::Error::last_os_error());
    }

    let owner = unsafe { OsString::from_wide(std::slice::from_raw_parts(owner_str, libc::wcslen(owner_str))) };
    unsafe {
        winapi::um::winbase::LocalFree(owner_str as *mut _);
    }

    Ok(owner)
}

pub fn get_file_owner(path: &Path) -> io::Result<OsString> {
    #[cfg(target_family = "unix")]
    {
        get_owner_unix(path)
    }

    #[cfg(target_family = "windows")]
    {
        get_owner_windows(path)
    }
}


// Easy file creation
pub fn create_file(path: &str) -> io::Result<File> {
    File::create(path)
}

// Easy file delete
pub fn open_file(path: &str) -> io::Result<File> {
    File::open(path)
}

// Easy file read
pub fn read_file(path: &str) -> io::Result<String> {
    let mut file = open_file(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Easy file rewriting
pub fn write_file(path: &str, content: &str, mode: WriteMode) -> io::Result<()> {
    let mut file = match mode {
        WriteMode::Append => OpenOptions::new().append(true).open(path)?,
        WriteMode::Overwrite => File::create(path)?,
        WriteMode::Delete(ref to_delete) => {
            let mut contents = read_file(path)?;
            contents = contents.replace(to_delete, ""); // Удаляем целевой текст
            contents = contents.replace("  ", " "); // Удаляем лишние пробелы
            let mut file = File::create(path)?;
            file.write_all(contents.as_bytes())?;
            return Ok(());
        }
    };
    file.write_all(content.as_bytes())?;
    Ok(())
}


pub fn has_file_access<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    // Проверка на существование файла
    if !path.exists() {
        return false;
    }

    // Проверка прав доступа на чтение и запись
    let metadata = match metadata(path) {
        Ok(meta) => meta,
        Err(_) => return false,
    };

    // Проверка прав на чтение
    let can_read = metadata.permissions().readonly() == false;

    // Проверка прав на запись
    let can_write = !metadata.permissions().readonly();

    can_read && can_write
}


pub fn get_file_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let meta = metadata(path)?;
    Ok(meta.len())
}

pub fn get_file_creation_date<P: AsRef<Path>>(path: P) -> io::Result<std::time::SystemTime> {
    let meta = metadata(path)?;
    let creation_time = meta.created()?;
    Ok(creation_time)
}

pub fn get_file_modification_date<P: AsRef<Path>>(path: P) -> io::Result<std::time::SystemTime> {
    let meta = metadata(path)?;
    let modification_time = meta.modified()?;
    Ok(modification_time)
}


pub fn has_directory_access<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    // Проверка на существование директории
    if !path.exists() || !path.is_dir() {
        return false;
    }

    // Получение метаданных директории
    let metadata = match metadata(path) {
        Ok(meta) => meta,
        Err(_) => return false,
    };

    // Проверка прав на чтение
    let can_read = metadata.permissions().readonly() == false;

    // Права на запись можно проверить через проверку на изменение
    let can_write = !metadata.permissions().readonly();

    can_read && can_write
}


// Easy directory creation
pub fn create_directory(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}

// Easy directory deletion
pub fn delete_directory(path: &str) -> io::Result<()> {
    if Path::new(path).exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

// Recording modes for write_file
pub enum WriteMode {
    Append,
    Overwrite,
    Delete(String),
}

pub fn move_directory(source: &str, destination: &str) -> io::Result<()> {
    fs::rename(source, destination)
}

// Retrieving the contents of a directory
pub fn get_directory_contents(path: &str) -> io::Result<Vec<PathBuf>> {
    let mut contents = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        contents.push(entry.path());
    }
    Ok(contents)
}


