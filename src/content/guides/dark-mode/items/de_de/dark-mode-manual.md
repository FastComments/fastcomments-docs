### Für Entwickler - Dunkelmodus erzwungen deaktivieren

Das erzwungene Deaktivieren des Dunkelmodus kann durch Übergabe von `hasDarkBackground` als `false` in der Widget-Konfiguration erfolgen. Dies funktioniert für die VanillaJS-, Angular-, React-, Vue- und React Native-Bibliotheken.

Jede Bibliothek hat einen `examples`-Ordner auf [GitHub](https://github.com/fastComments/), der Beispiele zur Verwendung des Dunkelmodus enthält.

### Dunkelmodus erzwungen aktivieren

Wir können den Dunkelmodus erzwingen, indem wir `hasDarkBackground` auf `true` setzen.

Wir können dies auch über die Widget-Anpassungs-UI [hier](https://fastcomments.com/auth/my-account/customize-widget) tun.

Unter `Basis-Theme` wählen Sie einfach `Dunkelmodus erzwingen`.

### VanillaJS Widget - Dunkelmodus aktualisieren

Der einfachste Weg, den Dunkelmodus zu aktualisieren, besteht darin, alle Instanzen des Widgets auf der Seite durchzugehen und ihre Konfiguration zu aktualisieren:

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
