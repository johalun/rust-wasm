extern crate webplatform;

fn main() {
    let document = webplatform::init();

    let body = document.element_query("body").expect("get body");
    body.html_append(
        "\
        <h1>This header is brought to you by Rust</h1>\
        <button>Click me!</button>\
    ",
    );

    let button = document.element_query("button").expect("get button");
    button.on("mouseenter", move |_| {
        println!("Mouse entered!");
        body.html_append("<p>Mouse entered!</p>");
    });

    // first 'body' moved into previous closure, get it again.
    let body = document.element_query("body").expect("get body again");
    button.on("mouseleave", move |_| {
        println!("Mouse left!");
        body.html_append("<p>Mouse left!</p>");
    });

    button.on("click", |_| {
        println!("Clicked!");
        webplatform::alert("Clicked!");
    });

    webplatform::spin();
}
