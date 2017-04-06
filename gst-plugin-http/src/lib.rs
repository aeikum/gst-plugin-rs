// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type="cdylib"]

extern crate url;
#[macro_use]
extern crate gst_plugin;
extern crate reqwest;
#[macro_use]
extern crate slog;

use gst_plugin::plugin::*;
use gst_plugin::source::*;

mod httpsrc;

use httpsrc::HttpSrc;

fn plugin_init(plugin: &Plugin) -> bool {
    source_register(plugin,
                    &SourceInfo {
                        name: "rshttpsrc",
                        long_name: "HTTP/HTTPS Source",
                        description: "Reads HTTP/HTTPS streams",
                        classification: "Source/File",
                        author: "Sebastian Dröge <sebastian@centricular.com>",
                        rank: 256 + 100,
                        create_instance: HttpSrc::new_boxed,
                        protocols: "http:https",
                        push_only: true,
                    });

    true
}

plugin_define!(b"rshttp\0",
               b"Rust HTTP Plugin\0",
               plugin_init,
               b"1.0\0",
               b"MIT/X11\0",
               b"rshttp\0",
               b"rshttp\0",
               b"https://github.com/sdroege/rsplugin\0",
               b"2016-12-08\0");