use crate::i18n::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_darkmode::Darkmode;

struct School {
    default_name: String,
    name_languages: HashMap<Locale, String>
}

struct Staff {
    default_name: String,
    pronouns: String,
    name_languages: HashMap<Locale, String>,
    team: Vec<TeamNames>,
    email: Option<String>,
    website: Option<String>,
    position_languages: HashMap<Locale, String>,
    default_position: String,
    school: Option<School>,
    picture_url: String
}

lazy_static! {
    static ref UCI: School = {
        let mut names = HashMap::new();

        names.insert(Locale::en, "University of California, Irvine");
        names.insert(Locale::zh, "加利福尼亞大學爾灣分校");
        names.insert(Locale::zh_TW, "加利福尼亞大學爾灣分校");
        names.insert(Locale::ko, "캘리포니아 대학교 어바인");
        names.insert(Locale::fr, "Université de Californie à Irvine");
        names.insert(Locale::es, "Universidad de California en Irvine");

        School {
            default_name: String::from("University of California, Irvine"),
            name_languages:: uci_langs
        }
    }

    static ref USC: School = {
        let mut names = HashMap::new();

        names.insert(Locale::en, "University of California, Irvine");
        names.insert(Locale::zh, "加利福尼亞大學爾灣分校");
        names.insert(Locale::zh_TW, "加利福尼亞大學爾灣分校");
        names.insert(Locale::ko, "서던캘리포니아 대학교");
        names.insert(Locale::fr, "Université de Californie à Irvine");
        names.insert(Locale::es, "Universidad de California en Irvine");

        School {
            default_name: String::from("University of California, Irvine"),
            name_languages:: uci_langs
        }
    }
}

#[component]
pub fn People() -> impl IntoView {

    leptos_meta::provide_meta_context();

    let mut darkmode = expect_context::<Darkmode>();

    provide_i18n_context();

    let i18n = use_i18n();

    let (opened, set_opened) = create_signal(false);

    view! {
    }
}