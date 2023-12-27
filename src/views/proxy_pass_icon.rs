use dioxus::prelude::*;

#[component]
pub fn ProxyPassIcon(cx: Scope, proxy_pass: bool, height: i32) -> Element {
    if *proxy_pass {
        render! {
            img {
                style: "max-height:{height}px",
                src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxMDQgMzkuNSI+PGRlZnM+PHN0eWxlPi5jbHMtMXtmaWxsOiM5OTk7fS5jbHMtMntmaWxsOiNmNjhhMWQ7fS5jbHMtM3tmaWxsOiNmZmY7fTwvc3R5bGU+PC9kZWZzPjx0aXRsZT5Bc3NldCAxPC90aXRsZT48ZyBpZD0iTGF5ZXJfMiIgZGF0YS1uYW1lPSJMYXllciAyIj48ZyBpZD0iTGF5ZXJfMS0yIiBkYXRhLW5hbWU9IkxheWVyIDEiPjxwb2x5Z29uIGNsYXNzPSJjbHMtMSIgcG9pbnRzPSIxMDQgMjAuMTIgOTQgMTAuNjIgOTQgMTYuMTIgMCAxNi4xMiAwIDI0LjEyIDk0IDI0LjEyIDk0IDI5LjYyIDEwNCAyMC4xMiIvPjxwYXRoIGNsYXNzPSJjbHMtMiIgZD0iTTc0LjUsMzljLTIuMDgsMC0xNS40My0uMTMtMjguMzQtLjI1LTEyLjYyLS4xMi0yNS42OC0uMjUtMjcuNjYtLjI1YTgsOCwwLDAsMS0xLTE1LjkzYzAtLjE5LDAtLjM4LDAtLjU3YTkuNDksOS40OSwwLDAsMSwxNC45LTcuODEsMTkuNDgsMTkuNDgsMCwwLDEsMzguMDUsNC42M0ExMC41LDEwLjUsMCwxLDEsNzQuNSwzOVoiLz48cGF0aCBjbGFzcz0iY2xzLTMiIGQ9Ik01MSwxQTE5LDE5LDAsMCwxLDcwLDE5LjU5LDEwLDEwLDAsMSwxLDc0LjUsMzguNWMtNC4xMSwwLTUyLS41LTU2LS41YTcuNSw3LjUsMCwwLDEtLjQ0LTE1QTguNDcsOC40NywwLDAsMSwxOCwyMmE5LDksMCwwLDEsMTQuNjgtN0ExOSwxOSwwLDAsMSw1MSwxbTAtMUEyMCwyMCwwLDAsMCwzMi4xMywxMy40MiwxMCwxMCwwLDAsMCwxNywyMnYuMTRBOC41LDguNSwwLDAsMCwxOC41LDM5YzIsMCwxNSwuMTMsMjcuNjYuMjUsMTIuOTEuMTIsMjYuMjYuMjUsMjguMzQuMjVhMTEsMTEsMCwxLDAtMy42MS0yMS4zOUEyMC4xLDIwLjEsMCwwLDAsNTEsMFoiLz48L2c+PC9nPjwvc3ZnPg=="
            }
            "Proxy pass"
        }
    } else {
        render! {
            img {
                style: "max-height:{height}px",
                src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA5MC41IDU5Ij48ZGVmcz48c3R5bGU+LmNscy0xe2ZpbGw6IzkyOTc5Yjt9PC9zdHlsZT48L2RlZnM+PHRpdGxlPkFzc2V0IDI8L3RpdGxlPjxnIGlkPSJMYXllcl8yIiBkYXRhLW5hbWU9IkxheWVyIDIiPjxnIGlkPSJMYXllcl8xLTIiIGRhdGEtbmFtZT0iTGF5ZXIgMSI+PHBhdGggY2xhc3M9ImNscy0xIiBkPSJNNDksMTMuNVYxOUw1OSw5LjUsNDksMFY1LjVINDAuNzhhMTIuNDMsMTIuNDMsMCwwLDAtOS41LDQuNDJMMTcuNjUsMjcuMTZhOC44Myw4LjgzLDAsMCwxLTYuOTEsMy4zNEg1bC01LDhIMTMuMzlhMTEuMjcsMTEuMjcsMCwwLDAsOS00LjQ4TDM1LjA1LDE3LjE4YTkuODEsOS44MSwwLDAsMSw3LjY2LTMuNjhaIi8+PHBhdGggY2xhc3M9ImNscy0xIiBkPSJNODAuNSwzOUExMCwxMCwwLDAsMCw3Niw0MC4wOWExOSwxOSwwLDAsMC0zNy4zLTQuNTdBOSw5LDAsMCwwLDI0LDQyLjVhOC40Nyw4LjQ3LDAsMCwwLC4wNiwxLDcuNSw3LjUsMCwwLDAsLjQ0LDE1YzQsMCw1MS44OS41LDU2LC41YTEwLDEwLDAsMCwwLDAtMjBaIi8+PC9nPjwvZz48L3N2Zz4="
            }
            "DNS Only"
        }
    }
}
