#[derive(Debug, Clone)]
pub struct UsbConstants;

impl UsbConstants {
    pub const USB_DIR_OUT: i32 = 0;
    pub const USB_DIR_IN: i32 = 0x80;
    pub const USB_ENDPOINT_XFER_INT: i32 = 3;
}
