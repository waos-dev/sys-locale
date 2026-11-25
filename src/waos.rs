//! WaOS: the languages the user set, as WaOS tells an app, most preferred first.

use alloc::string::String;

use wasdk::system::{self, Locale};

pub(crate) fn get() -> impl Iterator<Item = String> {
    system::locales().into_iter().map(tag)
}

/// `locale` as a BCP 47 tag: `en`, `zh-Hant-TW`.
fn tag(locale: Locale) -> String {
    let mut tag = locale.language;
    for subtag in locale.script.into_iter().chain(locale.region) {
        tag.push('-');
        tag.push_str(&subtag);
    }
    tag
}
