### За програмере - Присилно искључивање тамног режима

Присилно искључивање тамног режима може се урадити прослеђивањем `hasDarkBackground` као `false` у конфигурацији виџета. Ово ради за библиотеке VanillaJS, Angular, React, Vue и React Native.

Свака библиотека има фасциклу `examples` на [GitHub-у](https://github.com/fastComments/) која садржи примере како користити тамни режим.

### Присилно укључивање тамног режима

Можемо присилити да тамни режим увек буде укључен постављањем `hasDarkBackground` на `true`.

Ово можемо урадити и преко корисничког интерфејса за прилагођавање виџета [овде](https://fastcomments.com/auth/my-account/customize-widget).

Под `Base Theme` једноставно изаберите `Force Dark Mode`.

### VanillaJS виџет - Ажурирање тамног режима

Најлакши начин за ажурирање тамног режима је да прођете кроз све инстанце виџета на страници и ажурирате њихову конфигурацију:

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
