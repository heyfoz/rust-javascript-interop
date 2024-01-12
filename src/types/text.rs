use serde::Deserialize;

// Defines a structure to hold language-specific text and available languages
#[derive(Default, Clone, PartialEq)]
pub struct LanguageText {
    pub text: Text, // Holds the actual text strings for a specific language
    pub language_available: LanguagesAvailable // Enum for available languages
}

// Enum to represent the available languages in the application
#[derive(Clone, Copy, PartialEq)]
pub enum LanguagesAvailable {
    EN, // English
    ES, // Spanish
}

// Default implementation for LanguagesAvailable, defaulting to English
impl Default for LanguagesAvailable {
    fn default() -> Self {
        Self::EN
    }
}

// Structure representing texts for all supported languages
// Deserialized from a JSON file like 'text1.json'
#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub struct Languages {
    pub en: Text, 
    pub es: Text,
}

// Represents text strings defined in the text1.json file
#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub struct Text {
  pub string_1: String, // First string
  pub string_2: String, // Second string
  pub string_3: String, // Third string
}
