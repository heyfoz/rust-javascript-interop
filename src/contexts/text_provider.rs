use yew::prelude::*;
use reqwasm::http::Request;

use crate::types::texts::Languages;
use crate::types::texts::LanguagesAvailable;
use crate::types::texts::LanguageText;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children
}

#[function_component(TextProvider)]
pub fn text_provider(props: &Props) -> Html {

    let default_language = {
        let navigator = web_sys::window().unwrap().navigator();
        let language = navigator.language().unwrap();
        match language.as_str() {
            "es" | "es-ES" => LanguagesAvailable::ES,
            _ => LanguagesAvailable::EN, // Default to English
        }
    }; 

    let languages = use_state(|| LanguageText {
        texts: Default::default(),
        language_available: default_language
    });

    let languages_available = languages.language_available.clone();
    {
        let languages = languages.clone();
        use_effect_with_deps(move |_| {
            let languages = languages.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let text_request: Languages = Request::get("/text1.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                match languages.language_available {
                    LanguagesAvailable::EN => {
                        languages.set(LanguageTexts { 
                            texts: texts_request.en,
                            language_available: languages.language_available 
                        })
                    },
                    LanguagesAvailable::ES => languages.set(LanguageText { 
                        text: text_request.es,
                        language_available: languages.language_available
                }
            });
            || () 
        }, languages_available);
    }

    html! {
        <ContextProvider<UseStateHandle<LanguageText>> context={languages.clone()}>
            { props.children.clone() }
        </ContextProvider<UseStateHandle<LanguageText>>>
    }
}
