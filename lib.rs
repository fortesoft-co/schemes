//! The tinted-theming scheme collection (MIT), embedded at build time.
//! Maintained as a packaging layer on top of the base16/base24/tinted8
//! branches of this repository; contents stay the upstream files.

include!(concat!(env!("OUT_DIR"), "/schemes_table.rs"));
