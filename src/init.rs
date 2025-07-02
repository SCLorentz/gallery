use std::{ffi::CString, sync::LazyLock};

unsafe extern "C" {
    fn create_image(img_path: *const i8, size_mb: i32) -> i32;
    fn format_apfs(device: *const i8, volume_name: *const i8) -> i32;
    fn write_image(mount_point: *const i8, filename: *const i8, content: *const i8) -> i32;
    fn detach_image(device_buf: *mut i8) -> i32;
    fn attach_image(img_path: *const i8, device_buf: *mut i8, buf_len: usize) -> i32;
}

static VOLUME_NAME: LazyLock<CString> = LazyLock::new(|| CString::new("BHG").unwrap());

pub fn init_bhg()
{
    let img_path = CString::new("bhg.img").unwrap();
    let mount_point = CString::new(format!("/Volumes/{}", VOLUME_NAME.to_str().unwrap())).unwrap();
    let filename = CString::new("hello.txt").unwrap();
    let content = CString::new("Hello, APFS!").unwrap();

    unsafe
    {
        create_image(img_path.as_ptr(), 100);

        let mut device_buf = [0i8; 1024];
        attach_image(img_path.as_ptr(), device_buf.as_mut_ptr(), device_buf.len());

        format_apfs(device_buf.as_ptr(), VOLUME_NAME.as_ptr());

        write_image(mount_point.as_ptr(), filename.as_ptr(), content.as_ptr());

        detach_image(device_buf.as_mut_ptr());
    }
}