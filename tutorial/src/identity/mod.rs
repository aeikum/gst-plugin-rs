// Copyright (C) 2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// SPDX-License-Identifier: MIT/Apache-2.0

use gst::glib;
use gst::prelude::*;

mod imp;

// The public Rust wrapper type for our element
glib::wrapper! {
    pub struct Identity(ObjectSubclass<imp::Identity>) @extends gst::Element, gst::Object;
}

// Registers the type for our element, and then registers in GStreamer under
// the name "rsidentity" for being able to instantiate it via e.g.
// gst::ElementFactory::make().
pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "rsidentity",
        gst::Rank::None,
        Identity::static_type(),
    )
}
