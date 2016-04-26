// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use Action;
use ffi;
use glib;
use glib::translate::*;

glib_wrapper! {
    pub struct SimpleAction(Object<ffi::GSimpleAction>): Action;

    match fn {
        get_type => || ffi::g_simple_action_get_type(),
    }
}

impl SimpleAction {
    pub fn new(name: &str, parameter_type: Option<&glib::VariantTy>) -> SimpleAction {
        unsafe {
            from_glib_full(ffi::g_simple_action_new(name.to_glib_none().0, parameter_type.to_glib_none().0))
        }
    }

    pub fn new_stateful(name: &str, parameter_type: Option<&glib::VariantTy>, state: &glib::Variant) -> SimpleAction {
        unsafe {
            from_glib_full(ffi::g_simple_action_new_stateful(name.to_glib_none().0, parameter_type.to_glib_none().0, state.to_glib_none().0))
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::g_simple_action_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_state(&self, value: &glib::Variant) {
        unsafe {
            ffi::g_simple_action_set_state(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_44")]
    pub fn set_state_hint(&self, state_hint: Option<&glib::Variant>) {
        unsafe {
            ffi::g_simple_action_set_state_hint(self.to_glib_none().0, state_hint.to_glib_none().0);
        }
    }
}