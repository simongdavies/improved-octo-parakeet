use std::ffi::CString;

extern "C" {
    fn HostPrint(s: *const i8) -> i32;
}

#[no_mangle]
pub extern "C" fn Run() -> i32 {
    let grace_hopper_image = include_bytes!("grace_hopper.jpg");
    let mut message = CString::new("Loading image from memory\n").unwrap();
    unsafe {
        HostPrint(message.as_ptr());
    };

    let image = image::load_from_memory(grace_hopper_image)
        .unwrap()
        .to_rgb8();

    message = CString::new("Loaded image from memory\n").unwrap();
    unsafe {
        HostPrint(message.as_ptr());
    }

    message = CString::new("Resizing image\n").unwrap();

    unsafe {
        HostPrint(message.as_ptr());
    }

    let resized =
        image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    let x = resized.width();

    message = CString::new("Resized image\n").unwrap();

    unsafe {
        HostPrint(message.as_ptr());
    }

    x as i32
}
