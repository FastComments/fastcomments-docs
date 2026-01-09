In questo passaggio successivo è necessario copiare il codice del widget predefinito qui sotto.

Finché sei connesso a FastComments.com, lo snippet di codice qui sotto conterrà già le informazioni del tuo account. Copiamolo:

[inline-code-attrs-start title = 'Codice Super.so FastComments Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Pulisci l'istanza esistente
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Pulisci la top bar esistente se presente
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Crea una nuova top bar
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Inizializza FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Aggiorna il pathname corrente
            currentPathname = window.location.pathname;
        }

        // Caricamento iniziale
        load();

        // Controlla ogni 500ms per cambiamenti
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
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Ora incolla nell'area `Body`:

<div class="screenshot white-bg">
    <div class="title">Codice incollato</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Codice incollato" />
</div>

If you see a "this is a demo message" after pasting the code:

- Assicurati di aver effettuato l'accesso al tuo account su fastcomments.com.
- Assicurati che i cookie di terze parti siano abilitati.
- Quindi aggiorna questa pagina e copia nuovamente lo snippet di codice. Dovrebbe avere `tenantId` popolato con l'identificatore del tuo tenant.

---