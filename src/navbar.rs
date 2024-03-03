use crate::i18n::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_darkmode::Darkmode;

#[component]
pub fn Navbar() -> impl IntoView {

    leptos_meta::provide_meta_context();

    let mut darkmode = expect_context::<Darkmode>();

    provide_i18n_context();

    let i18n = use_i18n();

    let (opened, set_opened) = create_signal(false);

    view! {
        <header
        class="w-full z-30 md:bg-opacity-90 transition duration-300 ease-in-out bg-background backdrop-blur-sm shadow-lg"
    >
        <div class="max-w-6xl mx-auto px-5 sm:px-6">
            <div class="flex items-center justify-between h-16 md:h-20">
                <div class="shrink-0 mr-4">
                    <a href="/">
                        <span class="text-[1.3em]">
                            <img
                                src="/assets/images/favicon.png"
                                class="inline-block mr-4"
                                alt={"Catenary Logo"}
                                width={50}
                                height={30}
                            />
                            <span class="text-black dark:text-white">"Catenary"</span>
                        </span>
                    </a>
                </div>
                {/* Desktop navigation */}
                <nav class="flex grow">
                    {/* Desktop sign in links */}
                    <ul class="flex grow justify-end flex-wrap items-center">
                        <li class="md:order-2">
                            <a
                                href="https://maps.catenarymaps.org"
                                class="px-4 py-3 flex flex-row flex-grow-0 align-middle bg-slate-200 hover:bg-slate-300 dark:text-white dark:bg-[#141414] dark:text-white dark:hover:bg-gray-800 ml-4"
                            >
                                <span>{t!(i18n,launchmaps)}</span>
                                <svg
                                    class="w-3 h-3 fill-current text-gray-900 dark:text-gray-300 my-auto shrink-0 ml-2 -mr-1"
                                    viewBox="0 0 12 12"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path
                                        d="M11.707 5.293L7 .586 5.586 2l3 3H0v2h8.586l-3 3L7 11.414l4.707-4.707a1 1 0 000-1.414z"
                                        fillRule="nonzero"
                                    />
                                </svg>
                            </a>
                        </li>
                        <li class="hidden md:flex mb-4 md:ml-4 md:mb-0">
                           
                        </li>
                    </ul>
                </nav>
            </div>
        </div>
    </header>
        }
}

fn invert_bool(n: &mut bool) {
    if *n == false {
        *n = true;
    } else {
        *n = false;
    }
}

#[component]
fn PickTranslation(set_opened: WriteSignal<bool>) -> impl IntoView {
    leptos_meta::provide_meta_context();

    provide_i18n_context();

    let i18n = use_i18n();

    leptos_i18n::load_locales!();

    let avaliable_lang = vec![(Locale::en, "English"), (Locale::zh, "简体中文"), (Locale::zh_TW, "繁體中文"), (Locale::ko, "한국어")];

    view! {
        {
            avaliable_lang.into_iter().map(move |lang_pair| {
                view! {

                    <div on:click=move |_| {i18n.set_locale(lang_pair.0); set_opened.update(|n| *n = false);}>
                        {lang_pair.1}
                    </div>
                }
            })
            .collect_view()
        }
    }
}