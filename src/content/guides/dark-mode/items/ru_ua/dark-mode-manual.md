### Для разработчиков - Принудительное отключение тёмного режима

Принудительное отключение тёмного режима можно выполнить, передав `hasDarkBackground` как `false` в конфигурации виджета. Это работает для библиотек VanillaJS, Angular, React, Vue и React Native.

Каждая библиотека имеет папку `examples` на [GitHub](https://github.com/fastComments/), которая содержит примеры использования тёмного режима.

### Принудительное включение тёмного режима

Мы можем принудительно включить тёмный режим, установив `hasDarkBackground` в `true`.

Мы также можем сделать это через интерфейс настройки виджета [здесь](https://fastcomments.com/auth/my-account/customize-widget).

В разделе `Base Theme` просто выберите `Force Dark Mode`.

### Виджет VanillaJS - Обновление тёмного режима

Самый простой способ обновить тёмный режим — пройти через все экземпляры виджета на странице и обновить их конфигурацию:

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
