### Для розробників - Примусове вимкнення темного режиму

Примусове вимкнення темного режиму можна виконати, передавши `hasDarkBackground` як `false` у конфігурації віджета. Це працює для бібліотек VanillaJS, Angular, React, Vue та React Native.

Кожна бібліотека має папку `examples` на [GitHub](https://github.com/fastComments/), яка містить приклади використання темного режиму.

### Примусове ввімкнення темного режиму

Ми можемо примусово ввімкнути темний режим, встановивши `hasDarkBackground` у `true`.

Ми також можемо зробити це через інтерфейс налаштування віджета [тут](https://fastcomments.com/auth/my-account/customize-widget).

У розділі `Base Theme` просто виберіть `Force Dark Mode`.

### Віджет VanillaJS - Оновлення темного режиму

Найпростіший спосіб оновити темний режим — пройти через усі екземпляри віджета на сторінці та оновити їх конфігурацію:

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
