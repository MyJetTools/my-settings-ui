function set_focus(id) {
    setTimeout(function () {
        console.log("set focus: " + id);
        let el = document.getElementById(id);
        el.focus();
        el.select();
    }, 100);
}

function show_panel() {
    let el = document.getElementById("main");
    el.style.setProperty('--show-panel-offset', "var(--left-panel-data-width)");

}
function hide_panel() {
    let el = document.getElementById("main");
    el.style.setProperty('--show-panel-offset', "0px");
}




function scroll_to(id) {
    let el = document.getElementById(id);
    el.scrollIntoView();
}
