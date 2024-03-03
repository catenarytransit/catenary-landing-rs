use crate::i18n::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_darkmode::Darkmode;

use crate::navbar::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    let darkmode = Darkmode::init();

    view! {
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            

            <Html class=move || if darkmode.is_dark() { "dark" } else { "" }/>
            <Stylesheet id="leptos" href="/pkg/catenarylanding.css"/>
            <Stylesheet href="https://use.typekit.net/nhx0pgc.css"/>

            // content for this welcome page
            <Router>
                <main class="">
                    <Routes>
                        <Route path="" view=HomePage
                        ssr=SsrMode::Async
                        />
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
            </Router>
        }
}

#[component]
fn Footer() -> impl IntoView {
    leptos_meta::provide_meta_context();
    provide_i18n_context();
    let i18n = use_i18n();

    view! {
    <div class="w-full  px-8 py-4 text-sm  mt-auto">
        
    </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    leptos_meta::provide_meta_context();

    provide_i18n_context();

    let i18n = use_i18n();

    let mut darkmode = expect_context::<Darkmode>();

    view! {

        <Title text={t!(i18n, title)}/>
        // sets the document title
        <div
        class="bg-white dark:bg-catenarysea flex flex-col h-full"
        >

        <div class="w-full">
        <div class="bg-white dark:bg-catenarysea">
        <Navbar
        />

        <div>
        
        <div class="mt-32">
        <h1><span>"Catenary helps "</span><span>"connect"</span><span>" people and communities."</span></h1>
        </div>
            
        </div>

        </div>
        
        </div>
        <Footer/>
        </div>

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
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
