### Voor ontwikkelaars - Donkere modus geforceerd uitschakelen

Het geforceerd uitschakelen van donkere modus kan worden gedaan door `hasDarkBackground` als `false` door te geven in de widgetconfiguratie. Dit werkt voor de VanillaJS, Angular, React, Vue en React Native bibliotheken.

Elke bibliotheek heeft een `examples` map op [GitHub](https://github.com/fastComments/) die voorbeelden bevat over hoe donkere modus te gebruiken.

### Donkere modus geforceerd inschakelen

We kunnen donkere modus geforceerd altijd aan laten staan door `hasDarkBackground` op `true` te zetten.

We kunnen dit ook doen via de Widget Customization UI [hier](https://fastcomments.com/auth/my-account/customize-widget).

Onder `Base Theme` selecteert u gewoon `Force Dark Mode`.

### VanillaJS Widget - Donkere modus bijwerken

De gemakkelijkste manier om donkere modus bij te werken is door alle instanties van de widget op de pagina te doorlopen en hun configuratie bij te werken:

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
