use std::ffi::CString;

unsafe extern "C" {
    pub fn create_image(img_path: *const i8, size_mb: i32) -> i32;
    pub fn attach_image(img_path: *const i8, device_buf: *mut i8, buf_size: usize) -> i32;
    pub fn format_apfs(device: *const i8, volume_name: *const i8) -> i32;
    pub fn write_image(mount_point: *const i8, filename: *const i8, content: *const i8) -> i32;
    pub fn detach_image(device_buf: *mut i8) -> i32;
}

pub fn init_bhg()
{
    let img_path = CString::new("bhg.img").unwrap();
    let volume_name = CString::new("MyVolume").unwrap();
    let mount_point = CString::new("/Volumes/MyVolume").unwrap();
    let filename = CString::new("hello.txt").unwrap();
    let content = CString::new("Hello, APFS!").unwrap();

    unsafe {
        create_image(img_path.as_ptr(), 100);

        let mut device_buf = [0i8; 1024];
        attach_image(img_path.as_ptr(), device_buf.as_mut_ptr(), device_buf.len());

        format_apfs(device_buf.as_ptr(), volume_name.as_ptr());

        write_image(mount_point.as_ptr(), filename.as_ptr(), content.as_ptr());

        detach_image(device_buf.as_mut_ptr());

        println!("Image created and written successfully.");
    }
}