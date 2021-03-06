// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use glib::{Type, ToValue, Value};
use libc::c_int;
use TreeIter;
use TreeStore;

impl TreeStore {
    pub fn new(column_types: &[Type]) -> TreeStore {
        assert_initialized_main_thread!();
        unsafe {
            let mut column_types = column_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
            from_glib_full(
                ffi::gtk_tree_store_newv(column_types.len() as c_int,
                    column_types.as_mut_ptr()))
        }
    }

    pub fn reorder(&self, parent: &TreeIter, new_order: &[u32]) {
        unsafe {
            let count = ffi::gtk_tree_model_iter_n_children(self.to_glib_none().0,
                                                            mut_override(parent.to_glib_none().0));
            let safe_count = count as usize == new_order.len();
            debug_assert!(safe_count,
                          "Incorrect `new_order` slice length. Expected `{}`, found `{}`.",
                          count,
                          new_order.len());
            let safe_values = new_order.iter()
                .max()
                .map(|&max| {
                    let max = max as i32;
                    max >= 0 && max < count
                })
                .unwrap_or(true);
            debug_assert!(safe_values,
                          "Some `new_order` slice values are out of range. Maximum safe value: \
                           `{}`. The slice contents: `{:?}`",
                          count - 1,
                          new_order);
            if safe_count && safe_values {
                ffi::gtk_tree_store_reorder(self.to_glib_none().0,
                                            mut_override(parent.to_glib_none().0),
                                            mut_override(new_order.as_ptr() as *const c_int));
            }
        }
    }

    pub fn set(&self, iter: &TreeIter, columns: &[u32], values: &[&ToValue]) {
        unsafe {
            let n_columns = ffi::gtk_tree_model_get_n_columns(self.to_glib_none().0) as u32;
            assert!(columns.len() == values.len());
            assert!(columns.len() <= n_columns as usize);
            for &column in columns {
                assert!(column < n_columns);
                let type_ = from_glib(
                    ffi::gtk_tree_model_get_column_type(self.to_glib_none().0, column as c_int));
                assert!(Value::type_transformable(values[column as usize].to_value_type(), type_));
            }
            ffi::gtk_tree_store_set_valuesv(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(columns.as_ptr() as *const c_int),
                values.to_glib_none().0,
                columns.len() as c_int);
        }
    }

    pub fn set_value(&self, iter: &TreeIter, column: u32, value: &Value) {
        unsafe {
            let columns = ffi::gtk_tree_model_get_n_columns(self.to_glib_none().0);
            assert!(column < columns as u32);
            let type_ = from_glib(
                ffi::gtk_tree_model_get_column_type(self.to_glib_none().0, column as c_int));
            assert!(Value::type_transformable(value.type_(), type_));
            ffi::gtk_tree_store_set_value(self.to_glib_none().0,
                mut_override(iter.to_glib_none().0), column as c_int,
                mut_override(value.to_glib_none().0));
        }
    }
}
