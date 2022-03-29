pub mod frame_control;
pub mod generic;
pub mod mac_frame;

#[allow(unused_imports)]
use generic::*;
use mac_frame::{AddrNone, PanNone};

pub fn test() -> frame_control::W {
    let mut v = frame_control::W::new(0);
    v.frame_type()
        .data()
        .ack_request()
        .ack_requested()
        .dest_addr_mode()
        .address_16bit();
    v
}

pub fn test2() {
    let _ = mac_frame::MhrDefault::new();
    let mut v = mac_frame::Mhr::<PanNone, AddrNone, PanNone, AddrNone>::new();
    v.frame_control.modify(|v| {
        v.ack_request()
            .ack_not_requested()
            .dest_addr_mode()
            .address_16bit()
            .intra_pan()
            .inter_pan()
            .source_addr_mode()
            .not_present()
    });
}