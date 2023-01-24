// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

use crate::content_info::ContentInfo;
use crate::interface_info::InterfaceInfo;

pub trait ComponentInfo: ContentInfo {
    /// Returns the interface, the component uses the term "schema" to refer to it.
    fn schema(&self) -> &Option<Box<dyn InterfaceInfo>>;
}
