use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FlipperAction {
    Flip,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FlipperEvent {
    FlippedTo(u8),
}
