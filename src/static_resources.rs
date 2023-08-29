use std::net::SocketAddr;

pub const BOOTSTRAP_CSS: &[u8] = std::include_bytes!("../files/styled.css");
pub const APP_CSS: &[u8] = std::include_bytes!("../files/app.css");
pub const APP_JS: &[u8] = std::include_bytes!("../files/app.js");

pub fn get_header_content() -> String {
    let app_js_str = std::str::from_utf8(APP_JS).unwrap();
    let bootstrap_css_str = std::str::from_utf8(BOOTSTRAP_CSS).unwrap();
    let app_css_str = std::str::from_utf8(APP_CSS).unwrap();

    format!(
        r#"
        <link rel="icon" type="image/x-icon" href="/img/favicon.png">
    <script>
        {app_js_str}
    </script>
    
    <style>
        {bootstrap_css_str}
        {app_css_str}
    </style>
    "#
    )
}

pub fn get_html(addr: SocketAddr) -> String {
    let header_content = get_header_content();
    format!(
        r#"
            <!DOCTYPE html>
            <html>
                <head> <title>CFD web terminal</title> {header_content} </head>
                <body style:"--show-panel-offset:0"> <div id="main"></div> </body>
                {glue}
            </html>
            "#,
        glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
    )
}
