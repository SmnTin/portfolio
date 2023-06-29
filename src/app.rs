use yew::prelude::*;
use yew_router::prelude::*;

mod pages {
    use super::*;

    #[function_component(Home)]
    pub fn home() -> Html {
        html! {
            <div class="container">
                <h1 class="title">{ "Home" }</h1>
                <p>{ "This is the home page." }</p>
            </div>
        }
    }

    #[function_component(A)]
    pub fn a() -> Html {
        html! {
            <div class="container">
                <h1 class="title">{ "A" }</h1>
                <p>{ "This is the A page." }</p>
            </div>
        }
    }

    #[function_component(B)]
    pub fn b() -> Html {
        html! {
            <div class="container">
                <h1 class="title">{ "B" }</h1>
                <p>{ "This is the B page." }</p>
            </div>
        }
    }

    #[function_component(NotFound)]
    pub fn not_found() -> Html {
        html! {
            <div class="container">
                <h1 class="title">{ "404" }</h1>
                <p>{ "This is the 404 page." }</p>
            </div>
        }
    }
}

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[at("/a")]
    A,
    #[at("/b")]
    B,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <div class="navbar-start">
                <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                    { "Home" }
                </Link<Route>>
                { " | " }
                <Link<Route> classes={classes!("navbar-item")} to={Route::A}>
                    { "A" }
                </Link<Route>>
                { " | " }
                <Link<Route> classes={classes!("navbar-item")} to={Route::B}>
                    { "B" }
                </Link<Route>>
            </div>

            <main>
                <Switch<Route> render={switch} />
            </main>
            <footer class="footer">
            </footer>
        </HashRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::Home /> },
        Route::A => html! { <pages::A /> },
        Route::B => html! { <pages::B /> },
        Route::NotFound => html! { <pages::NotFound /> },
    }
}