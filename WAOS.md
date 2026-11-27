# sys-locale for WaOS

This fork adds WaOS to sys-locale 0.3.2: on a WaOS app (a WASI component, `target_os = "wasi"`) `get_locale` and `get_locales` answer the languages the user set, as WaOS tells an app, where upstream answers none. The change is `src/waos.rs`, its switch in `src/lib.rs` and the `wasdk` dependency for that target; everything else is upstream's. It is kept for whoever ports an app to WaOS, and is not offered upstream.

## Using it

```toml
[patch.crates-io]
sys-locale = { git = "https://github.com/waos-dev/sys-locale", branch = "waos" }
```

The fork asks for the published `wasdk` (0.8.0). An app that takes `wasdk` from WaOS's repository names it in the same `[patch.crates-io]`, so the fork builds with that one too.

## What it answers

The user's languages, most preferred first, each as a BCP 47 tag: `en`, `zh-Hant-TW`. A change while the app runs is `wasdk::system::changes`'s to tell; sys-locale has no way to.
