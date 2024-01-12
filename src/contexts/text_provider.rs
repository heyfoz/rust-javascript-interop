use yew::prelude::*;
use reqwasm::http::Request;

// Import custom types for handling language texts and available languages
use crate::types::texts::Languages;
use crate::types::texts::LanguagesAvailable;
use crate::types::texts::LanguageText;

// Define properties for the TextProvider component
#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children // Children elements to be rendered by this provider
}

// Define the TextProvider component for handling language texts
#[function_component(TextProvider)]
pub fn text_provider(props: &Props) -> Html {
    // Determine the default language from the browser's settings
    let default_language = {
        let navigator = web_sys::window().unwrap().navigator();
        let language = navigator.language().unwrap();
        match language.as_str() {
            "es" | "es-ES" => LanguagesAvailable::ES, // Spanish language settings
            _ => LanguagesAvailable::EN, // Default to English
        }
    };

    // State to manage the current language texts and availability
    let languages = use_state(|| LanguageText {
        text: Default::default(),
        language_available: default_language
    });

    // Clone state for use in fetching language data
    let languages_available = languages.language_available.clone();
    {
        let languages = languages.clone();
        // Fetch and set language data based on current language
        use_effect_with_deps(move |_| {
            let languages = languages.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Fetch language data from a JSON file
                let text_request: Languages = Request::get("/text1.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                // Set the language texts based on the fetched data
                match languages.language_available {
                    LanguagesAvailable::EN => {
                        languages.set(LanguageText { 
                            text: text_request.en,
                            language_available: languages.language_available 
                        })
                    },
                    LanguagesAvailable::ES => {
                        languages.set(LanguageText { 
                            text: text_request.es,
                            language_available: languages.language_available
                        })
                    }
                }
            });
            || () 
        }, languages_available);
    }

    // Provide the current language texts to the child components
    html! {
        <ContextProvider<UseStateHandle<LanguageText>> context={languages.clone()}>
            { props.children.clone() }
        </ContextProvider<UseStateHandle<LanguageText>>>
    }
}
