### Za programere - Prisilno isključivanje tamnog režima

Prisilno isključivanje tamnog režima može se uraditi prosleđivanjem `hasDarkBackground` kao `false` u konfiguraciji vidžeta. Ovo radi za biblioteke VanillaJS, Angular, React, Vue i React Native.

Svaka biblioteka ima fasciklu `examples` na [GitHub-u](https://github.com/fastComments/) koja sadrži primere kako koristiti tamni režim.

### Prisilno uključivanje tamnog režima

Možemo prisiliti da tamni režim uvek bude uključen postavljanjem `hasDarkBackground` na `true`.

Ovo možemo uraditi i preko korisničkog interfejsa za prilagođavanje vidžeta [ovde](https://fastcomments.com/auth/my-account/customize-widget).

Pod `Base Theme` jednostavno izaberite `Force Dark Mode`.

### VanillaJS vidžet - Ažuriranje tamnog režima

Najlakši način za ažuriranje tamnog režima je da prođete kroz sve instance vidžeta na stranici i ažurirate njihovu konfiguraciju:

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
