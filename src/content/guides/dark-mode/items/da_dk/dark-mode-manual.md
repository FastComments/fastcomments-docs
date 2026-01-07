### For udviklere - Tvinge mørk tilstand fra

At tvinge mørk tilstand fra kan gøres ved at sende `hasDarkBackground` som `false` i widget-konfigurationen. Dette virker for VanillaJS, Angular, React, Vue og React Native bibliotekerne.

Hvert bibliotek har en `examples` mappe på [GitHub](https://github.com/fastComments/), der indeholder eksempler på, hvordan man bruger mørk tilstand.

### Tvinge mørk tilstand til

Vi kan tvinge mørk tilstand til altid at være aktiveret ved at sætte `hasDarkBackground` til `true`.

Vi kan også gøre dette via Widget Customization UI [her](https://fastcomments.com/auth/my-account/customize-widget).

Under `Base Theme` vælg blot `Force Dark Mode`.

### VanillaJS Widget - Opdatering af mørk tilstand

Den nemmeste måde at opdatere mørk tilstand på er at gennemgå alle instanser af widgetten på siden og opdatere deres konfiguration:

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
