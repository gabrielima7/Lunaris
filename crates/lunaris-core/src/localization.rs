//! Localization System
//!
//! Multi-language support with RTL and pluralization.

use std::collections::HashMap;

/// Localization manager
pub struct Localization {
    pub current_locale: String,
    pub fallback_locale: String,
    pub strings: HashMap<String, LocaleData>,
    pub supported_locales: Vec<LocaleInfo>,
}

/// Locale info
pub struct LocaleInfo {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub rtl: bool,
    pub pluralization: PluralizationRule,
}

/// Locale data
pub struct LocaleData {
    pub strings: HashMap<String, String>,
    pub plurals: HashMap<String, PluralForms>,
}

/// Plural forms
pub struct PluralForms {
    pub zero: Option<String>,
    pub one: String,
    pub two: Option<String>,
    pub few: Option<String>,
    pub many: Option<String>,
    pub other: String,
}

/// Pluralization rule
pub enum PluralizationRule { English, French, Russian, Arabic, Japanese, Polish }

impl Localization {
    pub fn new() -> Self {
        Self {
            current_locale: "en".into(),
            fallback_locale: "en".into(),
            strings: HashMap::new(),
            supported_locales: Self::default_locales(),
        }
    }

    fn default_locales() -> Vec<LocaleInfo> {
        vec![
            LocaleInfo { code: "en".into(), name: "English".into(), native_name: "English".into(), rtl: false, pluralization: PluralizationRule::English },
            LocaleInfo { code: "pt-BR".into(), name: "Portuguese (Brazil)".into(), native_name: "Português (Brasil)".into(), rtl: false, pluralization: PluralizationRule::French },
            LocaleInfo { code: "es".into(), name: "Spanish".into(), native_name: "Español".into(), rtl: false, pluralization: PluralizationRule::French },
            LocaleInfo { code: "fr".into(), name: "French".into(), native_name: "Français".into(), rtl: false, pluralization: PluralizationRule::French },
            LocaleInfo { code: "de".into(), name: "German".into(), native_name: "Deutsch".into(), rtl: false, pluralization: PluralizationRule::English },
            LocaleInfo { code: "ja".into(), name: "Japanese".into(), native_name: "日本語".into(), rtl: false, pluralization: PluralizationRule::Japanese },
            LocaleInfo { code: "ko".into(), name: "Korean".into(), native_name: "한국어".into(), rtl: false, pluralization: PluralizationRule::Japanese },
            LocaleInfo { code: "zh-CN".into(), name: "Chinese (Simplified)".into(), native_name: "简体中文".into(), rtl: false, pluralization: PluralizationRule::Japanese },
            LocaleInfo { code: "ru".into(), name: "Russian".into(), native_name: "Русский".into(), rtl: false, pluralization: PluralizationRule::Russian },
            LocaleInfo { code: "ar".into(), name: "Arabic".into(), native_name: "العربية".into(), rtl: true, pluralization: PluralizationRule::Arabic },
            LocaleInfo { code: "he".into(), name: "Hebrew".into(), native_name: "עברית".into(), rtl: true, pluralization: PluralizationRule::English },
        ]
    }

    pub fn set_locale(&mut self, code: &str) -> Result<(), String> {
        if self.supported_locales.iter().any(|l| l.code == code) || self.strings.contains_key(code) {
            self.current_locale = code.into();
            Ok(())
        } else { Err("Unsupported locale".into()) }
    }

    pub fn get(&self, key: &str) -> String {
        self.get_for_locale(key, &self.current_locale)
            .or_else(|| self.get_for_locale(key, &self.fallback_locale))
            .unwrap_or_else(|| format!("[{}]", key))
    }

    fn get_for_locale(&self, key: &str, locale: &str) -> Option<String> {
        self.strings.get(locale)?.strings.get(key).cloned()
    }

    pub fn get_plural(&self, key: &str, count: i64) -> String {
        let locale = self.strings.get(&self.current_locale)
            .or_else(|| self.strings.get(&self.fallback_locale));
        
        if let Some(data) = locale {
            if let Some(forms) = data.plurals.get(key) {
                return self.select_plural_form(forms, count);
            }
        }
        format!("[{}:{}]", key, count)
    }

    fn select_plural_form(&self, forms: &PluralForms, count: i64) -> String {
        let info = self.supported_locales.iter().find(|l| l.code == self.current_locale);
        let rule = info.map(|i| &i.pluralization).unwrap_or(&PluralizationRule::English);

        match rule {
            PluralizationRule::English | PluralizationRule::German => {
                if count == 1 { forms.one.clone() } else { forms.other.clone() }
            }
            PluralizationRule::French => {
                if count == 0 || count == 1 { forms.one.clone() } else { forms.other.clone() }
            }
            PluralizationRule::Japanese => forms.other.clone(),
            PluralizationRule::Russian => {
                let n10 = count % 10;
                let n100 = count % 100;
                if n10 == 1 && n100 != 11 { forms.one.clone() }
                else if n10 >= 2 && n10 <= 4 && (n100 < 10 || n100 >= 20) { forms.few.clone().unwrap_or(forms.other.clone()) }
                else { forms.many.clone().unwrap_or(forms.other.clone()) }
            }
            PluralizationRule::Arabic => {
                if count == 0 { forms.zero.clone().unwrap_or(forms.other.clone()) }
                else if count == 1 { forms.one.clone() }
                else if count == 2 { forms.two.clone().unwrap_or(forms.other.clone()) }
                else if count % 100 >= 3 && count % 100 <= 10 { forms.few.clone().unwrap_or(forms.other.clone()) }
                else if count % 100 >= 11 { forms.many.clone().unwrap_or(forms.other.clone()) }
                else { forms.other.clone() }
            }
            PluralizationRule::Polish => {
                let n10 = count % 10;
                let n100 = count % 100;
                if count == 1 { forms.one.clone() }
                else if n10 >= 2 && n10 <= 4 && (n100 < 10 || n100 >= 20) { forms.few.clone().unwrap_or(forms.other.clone()) }
                else { forms.many.clone().unwrap_or(forms.other.clone()) }
            }
        }.replace("{count}", &count.to_string())
    }

    pub fn is_rtl(&self) -> bool {
        self.supported_locales.iter().find(|l| l.code == self.current_locale).map(|l| l.rtl).unwrap_or(false)
    }

    pub fn add_strings(&mut self, locale: &str, strings: HashMap<String, String>) {
        self.strings.entry(locale.into()).or_insert_with(|| LocaleData { strings: HashMap::new(), plurals: HashMap::new() }).strings.extend(strings);
    }
}

/// Macro for easy localization
#[macro_export]
macro_rules! t {
    ($key:expr) => { LOCALIZATION.get($key) };
    ($key:expr, $count:expr) => { LOCALIZATION.get_plural($key, $count) };
}
