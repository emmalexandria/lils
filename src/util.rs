use std::{ffi::CStr, mem, ptr};

use nix::unistd::{Gid, Uid};

// I know unsafe code sucks, but libc will be libc
pub fn get_user_by_uid(uid: u32) -> Option<String> {
    nix::unistd::User::from_uid(Uid::from_raw(uid))
        .ok()
        .flatten()
        .map(|u| u.name)
}

// This function is practically the same as the above
pub fn get_group_by_gid(gid: u32) -> Option<String> {
    nix::unistd::Group::from_gid(Gid::from_raw(gid))
        .ok()
        .flatten()
        .map(|g| g.name)
}
