use rust_extensions::StrOrString;

#[derive(Debug, Clone)]
pub enum ToastType {
    Info,
    Error,
}

pub fn show_toast<'s>(message: impl Into<StrOrString<'s>>, toast_type: ToastType) {
    let message = message.into();
    let toast_class = match toast_type {
        ToastType::Info => "toast bottom-0 start-0 text-bg-info",
        ToastType::Error => "toast bottom-0 start-0 text-bg-danger",
    };

    let js = format!(
        r#"

    document.getElementById('toast-message').innerText = "{}";
    let toast = document.getElementById('liveToast');
    toast.setAttribute('class', '{toast_class}');
    let toastBootstrap = bootstrap.Toast.getOrCreateInstance(toast);
    toastBootstrap.show();

"#,
        message.as_str()
    );

    let _ = dioxus_utils::js::eval(js.as_str());
}
