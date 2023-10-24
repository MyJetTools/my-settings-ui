//use dioxus_router_macro::Routable;
use crate::{Home, Secrets, Templates};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(dioxus_router_macro::Routable, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppRoute {
    #[route("/")]
    Home,
    #[route("/templates")]
    Templates,
    #[route("/secrets")]
    Secrets,
}
