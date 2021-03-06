// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use ScrollablePolicy;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Scrollable(Object<ffi::GtkScrollable>);

    match fn {
        get_type => || ffi::gtk_scrollable_get_type(),
    }
}

pub trait ScrollableExt {
    //#[cfg(feature = "3.16")]
    //fn get_border(&self, border: /*Ignored*/&mut Border) -> bool;

    fn get_hadjustment(&self) -> Option<Adjustment>;

    fn get_hscroll_policy(&self) -> ScrollablePolicy;

    fn get_vadjustment(&self) -> Option<Adjustment>;

    fn get_vscroll_policy(&self) -> ScrollablePolicy;

    fn set_hadjustment(&self, hadjustment: Option<&Adjustment>);

    fn set_hscroll_policy(&self, policy: ScrollablePolicy);

    fn set_vadjustment(&self, vadjustment: Option<&Adjustment>);

    fn set_vscroll_policy(&self, policy: ScrollablePolicy);
}

impl<O: IsA<Scrollable>> ScrollableExt for O {
    //#[cfg(feature = "3.16")]
    //fn get_border(&self, border: /*Ignored*/&mut Border) -> bool {
    //    unsafe { TODO: call ffi::gtk_scrollable_get_border() }
    //}

    fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_hadjustment(self.to_glib_none().0))
        }
    }

    fn get_hscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            ffi::gtk_scrollable_get_hscroll_policy(self.to_glib_none().0)
        }
    }

    fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_vadjustment(self.to_glib_none().0))
        }
    }

    fn get_vscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            ffi::gtk_scrollable_get_vscroll_policy(self.to_glib_none().0)
        }
    }

    fn set_hadjustment(&self, hadjustment: Option<&Adjustment>) {
        unsafe {
            ffi::gtk_scrollable_set_hadjustment(self.to_glib_none().0, hadjustment.to_glib_none().0);
        }
    }

    fn set_hscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_hscroll_policy(self.to_glib_none().0, policy);
        }
    }

    fn set_vadjustment(&self, vadjustment: Option<&Adjustment>) {
        unsafe {
            ffi::gtk_scrollable_set_vadjustment(self.to_glib_none().0, vadjustment.to_glib_none().0);
        }
    }

    fn set_vscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_vscroll_policy(self.to_glib_none().0, policy);
        }
    }
}
