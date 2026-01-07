### Za programere - Prisilno isključivanje tamnog načina rada

Prisilno isključivanje tamnog načina rada može se učiniti prosljeđivanjem `hasDarkBackground` kao `false` u konfiguraciji widgeta. Ovo radi za biblioteke VanillaJS, Angular, React, Vue i React Native.

Svaka biblioteka ima mapu `examples` na [GitHubu](https://github.com/fastComments/) koja sadrži primjere kako koristiti tamni način rada.

### Prisilno uključivanje tamnog načina rada

Možemo prisiliti da tamni način rada uvijek bude uključen postavljanjem `hasDarkBackground` na `true`.

To možemo učiniti i putem korisničkog sučelja za prilagodbu widgeta [ovdje](https://fastcomments.com/auth/my-account/customize-widget).

Pod `Base Theme` jednostavno odaberite `Force Dark Mode`.

### VanillaJS Widget - Ažuriranje tamnog načina rada

Najlakši način za ažuriranje tamnog načina rada je proći kroz sve instance widgeta na stranici i ažurirati njihovu konfiguraciju:

[inline-code-attrs-start title = 'VanillaJS - Dark Mode Toggle'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    let isDarkMode = somehowDetermineIfDarkModeEnabled();
    for (const instanceWrapped of window.fcUIInstances) {
        if (instanceWrapped.targetElement) {
            const config = instanceWrapped.config;
            config.hasDarkBackground = isDarkMode;
            instanceWrapped.instance.update(config)
        }
    }
[inline-code-end]
