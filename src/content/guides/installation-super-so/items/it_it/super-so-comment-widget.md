## Aggiungere un widget di commenti live ai tuoi articoli Notion su Super.so

Oltre a Collab Chat, puoi aggiungere un widget di commenti tradizionale alla parte inferiore dei tuoi articoli Notion. Questo permette ai lettori di lasciare commenti e discutere dell'intero articolo.

### Passaggi di installazione

Copia il seguente codice e incollalo nella sezione `Body` delle impostazioni del tuo sito Super.so:

[inline-code-attrs-start title = 'Widget di commenti live FastComments per Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Pulire l'istanza esistente
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Crea nuovo target
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Inizializza FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Aggiorna pathname corrente
            currentPathname = window.location.pathname;
        }

        // Caricamento iniziale
        load();

        // Controlla ogni 500ms per modifiche
        setInterval(() => {
            // Ricarica se il pathname è cambiato
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Ricarica se il widget è stato rimosso
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Ricarica se il contenitore è stato svuotato
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Note importanti

- Il widget dei commenti apparirà in fondo ai tuoi articoli Notion
- Ogni pagina avrà il proprio thread di commenti univoco basato sul percorso URL
- Assicurati di sostituire `"demo"` con il tuo tenant ID reale del tuo account FastComments
- Il widget gestisce automaticamente il caricamento dinamico delle pagine di Super.so

---