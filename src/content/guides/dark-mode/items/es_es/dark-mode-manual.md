### Para desarrolladores - Forzar la desactivación del modo oscuro

Forzar la desactivación del modo oscuro se puede hacer pasando `hasDarkBackground` como `false` en la configuración del widget. Esto funciona para las bibliotecas VanillaJS, Angular, React, Vue y React Native.

Cada biblioteca tiene una carpeta `examples` en [GitHub](https://github.com/fastComments/) que contiene ejemplos sobre cómo usar el modo oscuro.

### Forzar la activación del modo oscuro

Podemos forzar que el modo oscuro esté siempre activado estableciendo `hasDarkBackground` en `true`.

También podemos hacer esto a través de la interfaz de personalización del widget [aquí](https://fastcomments.com/auth/my-account/customize-widget).

Bajo `Base Theme` simplemente seleccione `Force Dark Mode`.

### Widget VanillaJS - Actualización del modo oscuro

La forma más fácil de actualizar el modo oscuro es recorrer todas las instancias del widget en la página y actualizar su configuración:

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
