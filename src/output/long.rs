use crate::{ConfigArgs, files::FsEntry};

// pub fn long(roots: &Vec<FsEntry>, config: &ConfigArgs) {
//     if config.recurse {
//         let all = roots.iter().map(|e| e.get_all_dirs());
//
//         all.for_each(|r| {
//             r.iter().enumerate().for_each(|(i, e)| {
//                 display_single(e, config, r.len(), i);
//             });
//         });
//     } else {
//         roots.iter().enumerate().for_each(|(i, e)| {
//             display_single(e, config, roots.len(), i);
//         });
//     }
// }
//
// fn display_single(entry: &FsEntry, config: &ConfigArgs) {}
//
// fn long_display() {}
