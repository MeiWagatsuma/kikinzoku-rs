use leptos::{
    component, create_resource, create_signal, expect_context, server, spawn_local, view, IntoView,
    ServerFnError, SignalGet, SignalUpdate,
};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // a signal that controls how many cat pics we want
    let (how_many_cats, set_how_many_cats) = create_signal(1);

    // create a resource that will refetch whenever `how_many_cats` changes
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <button on:click=move|_| set_how_many_cats.update(|count| *count +=1 )>"More Cats"</button>
        <p>"cats: " {how_many_cats}</p>
        <button on:click=move |_| {  spawn_local(async {
            say_hello_in_server().await;
        }) }>"hello"</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    // {
    //     // this can be done inline because it's synchronous
    //     // if it were async, we'd use a server function
    //     let resp = expect_context::<leptos_actix::ResponseOptions>();
    //     resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    // }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server]
pub async fn say_hello_in_server() -> Result<(), ServerFnError> {
    println!("hello");
    Err(ServerFnError::Request("hello".to_owned()))
    // Ok(())
}
