### Per sviluppatori - Forzare la disattivazione della modalità scura

Forzare la disattivazione della modalità scura può essere fatto passando `hasDarkBackground` come `false` nella configurazione del widget. Questo funziona per le librerie VanillaJS, Angular, React, Vue e React Native.

Ogni libreria ha una cartella `examples` su [GitHub](https://github.com/fastComments/) che contiene esempi su come usare la modalità scura.

### Forzare l'attivazione della modalità scura

Possiamo forzare la modalità scura ad essere sempre attiva impostando `hasDarkBackground` su `true`.

Possiamo anche farlo tramite l'interfaccia di personalizzazione del widget [qui](https://fastcomments.com/auth/my-account/customize-widget).

Sotto `Base Theme` seleziona semplicemente `Force Dark Mode`.

### Widget VanillaJS - Aggiornamento della modalità scura

Il modo più semplice per aggiornare la modalità scura è passare attraverso tutte le istanze del widget sulla pagina e aggiornare la loro configurazione:

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
