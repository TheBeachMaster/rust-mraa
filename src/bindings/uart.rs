/* automatically generated by rust-bindgen */

pub enum Struct__uart { }
pub type mraa_uart_context = *mut Struct__uart;
#[link(name = "mraa")]
extern "C" {
    pub fn mraa_uart_init(uart: ::libc::c_int) -> mraa_uart_context;
    pub fn mraa_uart_get_dev_path(dev: mraa_uart_context)
     -> *mut ::libc::c_char;
}
