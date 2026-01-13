### За разработчици - Принудително изключване на тъмен режим

Принудителното изключване на тъмен режим може да се направи, като се подаде `hasDarkBackground` като `false` в конфигурацията на уиджета. Това работи за библиотеките VanillaJS, Angular, React, Vue и React Native.

Всяка библиотека има папка `examples` в [GitHub](https://github.com/fastComments/), която съдържа примери как да използвате тъмен режим.

### Принудително включване на тъмен режим

Можем да принудим тъмния режим винаги да е включен, като зададем `hasDarkBackground` на `true`.

Можем също да направим това чрез потребителския интерфейс за персонализация на уиджета [тук](https://fastcomments.com/auth/my-account/customize-widget).

Под `Base Theme` просто изберете `Force Dark Mode`.

### VanillaJS Widget - Актуализиране на тъмен режим

Най-лесният начин за актуализиране на тъмния режим е да преминете през всички инстанции на уиджета на страницата и да актуализирате тяхната конфигурация:

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
