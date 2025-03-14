#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//!
//! A Rust library providing access to the Linux Media Subsystem.
//! It is implemented in the form of a wrapper around linux/media.h.
//!

use std::mem::size_of;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const fn _IOC(
    dir: libc::c_ulong,
    r#type: libc::c_char,
    nr: libc::c_ulong,
    size: libc::size_t,
) -> libc::c_ulong {
    (dir << _IOC_DIRSHIFT)
        | ((r#type as libc::c_ulong) << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | ((size as libc::c_ulong) << _IOC_SIZESHIFT)
}

const fn _IO(type_: libc::c_char, nr: libc::c_ulong) -> libc::c_ulong {
    _IOC(_IOC_NONE as libc::c_ulong, type_, nr, 0)
}
const fn _IOR(type_: libc::c_char, nr: libc::c_ulong, size: libc::size_t) -> libc::c_ulong {
    _IOC(_IOC_READ as libc::c_ulong, type_, nr, size)
}
const fn _IOW(type_: libc::c_char, nr: libc::c_ulong, size: libc::size_t) -> libc::c_ulong {
    _IOC(_IOC_WRITE as libc::c_ulong, type_, nr, size)
}
const fn _IOWR(type_: libc::c_char, nr: libc::c_ulong, size: libc::size_t) -> libc::c_ulong {
    _IOC((_IOC_READ | _IOC_WRITE) as libc::c_ulong, type_, nr, size)
}

pub const MEDIA_IOC_DEVICE_INFO: libc::c_ulong = _IOWR(b'|', 0x00, size_of::<media_device_info>());
pub const MEDIA_IOC_ENUM_ENTITIES: libc::c_ulong =
    _IOWR(b'|', 0x01, size_of::<media_entity_desc>());
pub const MEDIA_IOC_ENUM_LINKS: libc::c_ulong = _IOWR(b'|', 0x02, size_of::<media_links_enum>());
pub const MEDIA_IOC_SETUP_LINK: libc::c_ulong = _IOWR(b'|', 0x03, size_of::<media_link_desc>());
pub const MEDIA_IOC_G_TOPOLOGY: libc::c_ulong = _IOWR(b'|', 0x04, size_of::<media_v2_topology>());
pub const MEDIA_IOC_REQUEST_ALLOC: libc::c_ulong = _IOR(b'|', 0x05, size_of::<libc::c_int>());

///
/// These ioctls are called on the request file descriptor as returned
/// by MEDIA_IOC_REQUEST_ALLOC.
///
pub const MEDIA_REQUEST_IOC_QUEUE: libc::c_ulong = _IO(b'|', 0x80);
pub const MEDIA_REQUEST_IOC_REINIT: libc::c_ulong = _IO(b'|', 0x81);

///
/// Appeared in 4.19.0.
///
/// The media_version argument comes from the media_version field in
/// struct media_device_info.
///
pub const fn MEDIA_V2_ENTITY_HAS_FLAGS(media_version: u64) -> bool {
    return media_version >= ((4u64 << 16) | (19u64 << 8) | 0u64);
}

///
/// Appeared in 4.19.0.
///
/// The media_version argument comes from the media_version field in
/// struct media_device_info.
///
pub const fn MEDIA_V2_PAD_HAS_INDEX(media_version: u64) -> bool {
	return media_version >= ((4u64 << 16) | (19u64 << 8) | 0u64);
}
