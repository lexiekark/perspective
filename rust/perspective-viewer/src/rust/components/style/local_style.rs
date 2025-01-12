////////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) 2018, the Perspective Authors.
//
// This file is part of the Perspective library, distributed under the terms
// of the Apache License 2.0.  The full license can be found in the LICENSE
// file.

use super::style_cache::*;
use yew::prelude::*;

#[derive(Properties)]
pub struct LocalStyleProps {
    pub href: (&'static str, &'static str),
}

impl PartialEq for LocalStyleProps {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[function_component(LocalStyle)]
pub fn local_style(props: &LocalStyleProps) -> Html {
    use_context::<StyleCache>()
        .unwrap()
        .add_style(props.href.0, props.href.1);

    html! {}
}
