### Za razvijalce - Prisilno izključevanje temnega načina

Prisilno izključitev temnega načina lahko izvedete tako, da v konfiguraciji pripomočka podate `hasDarkBackground` kot `false`. To deluje za knjižnice VanillaJS, Angular, React, Vue in React Native.

Vsaka knjižnica ima mapo `examples` na [GitHubu](https://github.com/fastComments/), ki vsebuje primere uporabe temnega načina.

### Prisilno vključevanje temnega načina

Temni način lahko prisilimo, da je vedno vklopljen, tako da nastavimo `hasDarkBackground` na `true`.

To lahko storimo tudi prek uporabniškega vmesnika za prilagajanje pripomočka [tukaj](https://fastcomments.com/auth/my-account/customize-widget).

Pod `Base Theme` preprosto izberite `Force Dark Mode`.

### VanillaJS pripomoček - Posodabljanje temnega načina

Najlažji način za posodobitev temnega načina je, da greste skozi vse primerke pripomočka na strani in posodobite njihovo konfiguracijo:

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
