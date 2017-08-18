// This file was generated by gir (cfd99ec+) from gir-files (???)
// DO NOT EDIT

use Bus;
use Caps;
use Device;
use Object;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DeviceMonitor(Object<ffi::GstDeviceMonitor>): Object;

    match fn {
        get_type => || ffi::gst_device_monitor_get_type(),
    }
}

impl DeviceMonitor {
    pub fn new() -> DeviceMonitor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_device_monitor_new())
        }
    }
}

impl Default for DeviceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for DeviceMonitor {}
unsafe impl Sync for DeviceMonitor {}

pub trait DeviceMonitorExt {
    fn add_filter<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Caps>>>(&self, classes: P, caps: Q) -> u32;

    fn get_bus(&self) -> Option<Bus>;

    fn get_devices(&self) -> Vec<Device>;

    fn get_providers(&self) -> Vec<String>;

    fn get_show_all_devices(&self) -> bool;

    fn remove_filter(&self, filter_id: u32) -> bool;

    fn set_show_all_devices(&self, show_all: bool);

    fn start(&self) -> bool;

    fn stop(&self);

    fn get_property_show_all(&self) -> bool;

    fn set_property_show_all(&self, show_all: bool);

    fn connect_property_show_all_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DeviceMonitor> + IsA<glib::object::Object>> DeviceMonitorExt for O {
    fn add_filter<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Caps>>>(&self, classes: P, caps: Q) -> u32 {
        let classes = classes.into();
        let classes = classes.to_glib_none();
        let caps = caps.into();
        let caps = caps.to_glib_none();
        unsafe {
            ffi::gst_device_monitor_add_filter(self.to_glib_none().0, classes.0, caps.0)
        }
    }

    fn get_bus(&self) -> Option<Bus> {
        unsafe {
            from_glib_full(ffi::gst_device_monitor_get_bus(self.to_glib_none().0))
        }
    }

    fn get_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_monitor_get_devices(self.to_glib_none().0))
        }
    }

    fn get_providers(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_monitor_get_providers(self.to_glib_none().0))
        }
    }

    fn get_show_all_devices(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_monitor_get_show_all_devices(self.to_glib_none().0))
        }
    }

    fn remove_filter(&self, filter_id: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_device_monitor_remove_filter(self.to_glib_none().0, filter_id))
        }
    }

    fn set_show_all_devices(&self, show_all: bool) {
        unsafe {
            ffi::gst_device_monitor_set_show_all_devices(self.to_glib_none().0, show_all.to_glib());
        }
    }

    fn start(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_monitor_start(self.to_glib_none().0))
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::gst_device_monitor_stop(self.to_glib_none().0);
        }
    }

    fn get_property_show_all(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-all".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_all(&self, show_all: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-all".to_glib_none().0, Value::from(&show_all).to_glib_none().0);
        }
    }

    fn connect_property_show_all_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-all",
                transmute(notify_show_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_show_all_trampoline<P>(this: *mut ffi::GstDeviceMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceMonitor> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&DeviceMonitor::from_glib_none(this).downcast_unchecked())
}
