[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Per i siti che consentono di attivare/disattivare la modalità scura dopo il caricamento iniziale della pagina, questo è un po' più complesso.

Innanzitutto, tutte le versioni correnti della libreria del widget Comment (React, Vue) contengono esempi di attivazione/disattivazione della modalità scura nei rispettivi repository.

Per il widget VanillaJS, dovremo fare un po' più di lavoro. Innanzitutto, FastCommentsUI restituisce un oggetto con le funzioni "destroy" e "update".

Possiamo semplicemente chiamare la funzione update ogni volta che vogliamo aggiornare la configurazione del widget dei commenti, come segue. Ecco un esempio completo e funzionante di attivazione/disattivazione
della modalità scura con il widget VanillaJS.

[inline-code-attrs-start title = 'Esempio completo di attivazione/disattivazione della modalità scura'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---