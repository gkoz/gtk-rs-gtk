// This file was generated by gir (e0b4c3b) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Bin;
use Buildable;
use Container;
use Widget;

pub type FlowBoxChild = Object<ffi::GtkFlowBoxChild>;

unsafe impl Upcast<Widget> for FlowBoxChild { }
unsafe impl Upcast<Container> for FlowBoxChild { }
unsafe impl Upcast<Bin> for FlowBoxChild { }
unsafe impl Upcast<Buildable> for FlowBoxChild { }

impl FlowBoxChild {
    #[cfg(gtk_3_12)]
    pub fn new() -> FlowBoxChild {
        unsafe {
            Widget::from_glib_none(ffi::gtk_flow_box_child_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_12)]
    pub fn changed(&self) {
        unsafe {
            ffi::gtk_flow_box_child_changed(self.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_12)]
    pub fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_flow_box_child_get_index(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_12)]
    pub fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_child_is_selected(self.to_glib_none().0))
        }
    }

}

impl types::StaticType for FlowBoxChild {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_flow_box_child_get_type()) }
    }
}