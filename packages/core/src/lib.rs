#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/79236386")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/79236386")]
#![warn(missing_docs)]

mod any_props;
mod arena;
mod create;
mod diff;
mod dirty_scope;
mod error_boundary;
mod events;
mod fragment;
mod global_context;
mod mutations;
mod nodes;
mod properties;
mod runtime;
mod scheduler;
mod scope_arena;
mod scope_context;
mod scopes;
mod virtual_dom;

pub(crate) mod innerlude {
    pub use crate::arena::*;
    pub use crate::dirty_scope::*;
    pub use crate::error_boundary::*;
    pub use crate::events::*;
    pub use crate::fragment::*;
    pub use crate::global_context::*;
    pub use crate::mutations::*;
    pub use crate::nodes::RenderReturn;
    pub use crate::nodes::*;
    pub use crate::properties::*;
    pub use crate::runtime::{Runtime, RuntimeGuard};
    pub use crate::scheduler::*;
    pub use crate::scopes::*;
    pub use crate::virtual_dom::*;

    /// An [`Element`] is a possibly-none [`VNode`] created by calling `render` on [`Scope`] or [`ScopeState`].
    ///
    /// An Errored [`Element`] will propagate the error to the nearest error boundary.
    pub type Element = Option<VNode>;

    /// A [`Component`] is a function that takes a [`Scope`] and returns an [`Element`].
    ///
    /// Components can be used in other components with two syntax options:
    /// - lowercase as a function call with named arguments (rust style)
    /// - uppercase as an element (react style)
    ///
    /// ## Rust-Style
    ///
    /// ```rust, ignore
    /// fn example(cx: Scope<Props>) -> Element {
    ///     // ...
    /// }
    ///
    /// rsx!(
    ///     example()
    /// )
    /// ```
    /// ## React-Style
    /// ```rust, ignore
    /// fn Example(cx: Scope<Props>) -> Element {
    ///     // ...
    /// }
    ///
    /// rsx!(
    ///     Example {}
    /// )
    /// ```
    pub type Component<P = ()> = fn(P) -> Element;
}

pub use crate::innerlude::{
    fc_to_builder, generation, once, vdom_is_rendering, AnyValue, Attribute, AttributeValue,
    CapturedError, Component, DynamicNode, Element, ElementId, Event, Fragment, IntoDynNode,
    Mutation, MutationsVec, Properties, RenderReturn, ScopeId, Task, Template, TemplateAttribute,
    TemplateNode, VComponent, VNode, VPlaceholder, VText, VirtualDom, WriteMutations,
};

/// The purpose of this module is to alleviate imports of many common types
///
/// This includes types like [`Scope`], [`Element`], and [`Component`].
pub mod prelude {
    pub use crate::innerlude::{
        consume_context, consume_context_from_scope, current_scope_id, fc_to_builder, generation,
        has_context, needs_update, once, parent_scope, provide_context, provide_root_context,
        push_future, remove_future, spawn, spawn_forever, suspend, use_error_boundary, AnyValue,
        Component, Element, ErrorBoundary, Event, EventHandler, Fragment, IntoAttributeValue,
        IntoDynNode, Properties, Runtime, RuntimeGuard, ScopeId, Task, Template, TemplateAttribute,
        TemplateNode, Throw, VNode, VirtualDom,
    };
}
