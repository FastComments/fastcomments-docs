### Dla programistów - Wymuszanie wyłączenia trybu ciemnego

Wymuszenie wyłączenia trybu ciemnego można wykonać, przekazując `hasDarkBackground` jako `false` w konfiguracji widgetu. Działa to dla bibliotek VanillaJS, Angular, React, Vue i React Native.

Każda biblioteka ma folder `examples` na [GitHubie](https://github.com/fastComments/), który zawiera przykłady użycia trybu ciemnego.

### Wymuszanie włączenia trybu ciemnego

Możemy wymusić, aby tryb ciemny był zawsze włączony, ustawiając `hasDarkBackground` na `true`.

Możemy to również zrobić za pomocą interfejsu personalizacji widgetu [tutaj](https://fastcomments.com/auth/my-account/customize-widget).

W sekcji `Base Theme` wystarczy wybrać `Force Dark Mode`.

### Widget VanillaJS - Aktualizacja trybu ciemnego

Najłatwiejszym sposobem aktualizacji trybu ciemnego jest przejście przez wszystkie instancje widgetu na stronie i zaktualizowanie ich konfiguracji:

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
