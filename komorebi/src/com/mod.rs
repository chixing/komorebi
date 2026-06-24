// This code is largely taken verbatim from this repository: https://github.com/Ciantic/AltTabAccessor
// which the author Jari Pennanen (Ciantic) has kindly made available with the MIT license, available
// in full here: https://github.com/Ciantic/AltTabAccessor/blob/main/LICENSE.txt

mod interfaces;

use interfaces::CLSID_ImmersiveShell;
use interfaces::IApplicationViewCollection;
use interfaces::IServiceProvider;

use std::ffi::c_void;

use windows::Win32::Foundation::HWND;
use windows::Win32::System::Com::CLSCTX_ALL;
use windows::Win32::System::Com::COINIT_MULTITHREADED;
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::System::Com::CoInitializeEx;
use windows::Win32::System::Com::CoUninitialize;
use windows::core::GUID;
use windows_core::Interface;

struct ComInit();

impl ComInit {
    pub fn new() -> Self {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
        }
        Self()
    }
}

impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

thread_local! {
    static COM_INIT: ComInit = ComInit::new();
}

fn get_iservice_provider() -> IServiceProvider {
    COM_INIT.with(|_| unsafe { CoCreateInstance(&CLSID_ImmersiveShell, None, CLSCTX_ALL).unwrap() })
}

fn get_iapplication_view_collection(provider: &IServiceProvider) -> IApplicationViewCollection {
    COM_INIT.with(|_| {
        let mut obj = std::ptr::null_mut::<c_void>();
        unsafe {
            provider
                .query_service(
                    &IApplicationViewCollection::IID,
                    &IApplicationViewCollection::IID,
                    &mut obj,
                )
                .unwrap();
        }

        assert!(!obj.is_null());

        unsafe { IApplicationViewCollection::from_raw(obj) }
    })
}

pub fn virtual_desktop_id(hwnd: HWND) -> Option<Vec<u8>> {
    COM_INIT.with(|_| {
        let provider = get_iservice_provider();
        let view_collection = get_iapplication_view_collection(&provider);
        let mut view = None;

        if unsafe { view_collection.get_view_for_hwnd(hwnd, &mut view) }.is_err() {
            return None;
        }

        let view = view?;
        let mut desktop_id = GUID::zeroed();

        if unsafe { view.get_virtual_desktop_id(&mut desktop_id) }.is_err() {
            return None;
        }

        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(&desktop_id.data1.to_le_bytes());
        bytes.extend_from_slice(&desktop_id.data2.to_le_bytes());
        bytes.extend_from_slice(&desktop_id.data3.to_le_bytes());
        bytes.extend_from_slice(&desktop_id.data4);
        Some(bytes)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn SetCloak(hwnd: HWND, cloak_type: u32, flags: i32) {
    COM_INIT.with(|_| {
        let provider = get_iservice_provider();
        let view_collection = get_iapplication_view_collection(&provider);
        let mut view = None;
        unsafe {
            if view_collection.get_view_for_hwnd(hwnd, &mut view).is_err() {
                tracing::error!(
                    "could not get view for hwnd {} due to os error: {}",
                    hwnd.0 as isize,
                    std::io::Error::last_os_error()
                );
            }
        };

        view.map_or_else(
            || {
                tracing::error!("no view was found for {}", hwnd.0 as isize);
            },
            |view| {
                unsafe {
                    if view.set_cloak(cloak_type, flags).is_err() {
                        tracing::error!(
                            "could not change the cloaking status for hwnd {} due to os error: {}",
                            hwnd.0 as isize,
                            std::io::Error::last_os_error()
                        );
                    }
                };
            },
        );
    });
}
