use crate::{sapp_event, sapp_event_type_SAPP_EVENTTYPE_CUSTOM, sapp_context};

extern "C" {
    pub fn post_custom_event(event_data: *mut ::std::os::raw::c_void);
}

#[no_mangle]
pub extern "C" fn custom_event(event_data: *mut ::std::os::raw::c_void) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_CUSTOM;
    event.custom_data = event_data;
    unsafe {
        sapp_context().event(event);
    }

}