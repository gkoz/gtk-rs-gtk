// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureDrag;
use GestureSingle;
#[cfg(feature = "3.14")]
use Orientation;
#[cfg(feature = "3.14")]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct GesturePan(Object<ffi::GtkGesturePan>): EventController, Gesture, GestureSingle, GestureDrag;

    match fn {
        get_type => || ffi::gtk_gesture_pan_get_type(),
    }
}

impl GesturePan {
    #[cfg(feature = "3.14")]
    pub fn new<T: IsA<Widget>>(widget: &T, orientation: Orientation) -> GesturePan {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_pan_new(widget.to_glib_none().0, orientation)).downcast_unchecked()
        }
    }

    #[cfg(feature = "3.14")]
    pub fn get_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_gesture_pan_get_orientation(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "3.14")]
    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_gesture_pan_set_orientation(self.to_glib_none().0, orientation);
        }
    }
}
