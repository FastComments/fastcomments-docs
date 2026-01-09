Im nächsten Schritt müssen Sie den vorgefertigten Widget-Code unten kopieren.

Solange Sie bei FastComments.com eingeloggt sind, enthält der untenstehende Code-Snippet bereits Ihre Kontoinformationen. Kopieren Sie ihn:

[inline-code-attrs-start title = 'Super.so FastComments Kollaborations-Chat-Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Vorhandene Instanz bereinigen
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Vorhandene Top-Leiste bereinigen, falls vorhanden
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Neue Top-Leiste erstellen
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // FastComments Collab Chat initialisieren
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Aktuellen Pfad aktualisieren
            currentPathname = window.location.pathname;
        }

        // Erstinitialisierung
        load();

        // Prüfe alle 500 ms auf Änderungen
        setInterval(() => {
            // Neu laden, wenn sich der Pfad geändert hat
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Neu laden, wenn das Widget entfernt wurde
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Neu laden, wenn der Container geleert wurde
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Fügen Sie es jetzt im Bereich `Body` ein:

<div class="screenshot white-bg">
    <div class="title">Eingefügter Code</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Eingefügter Code" />
</div>

If you see a "this is a demo message" after pasting the code:

- Stellen Sie sicher, dass Sie in Ihrem fastcomments.com-Konto eingeloggt sind.
- Stellen Sie sicher, dass Drittanbieter-Cookies aktiviert sind.
- Aktualisieren Sie dann diese Seite und kopieren Sie den Code-Snippet erneut. Es sollte `tenantId` mit der Kennung Ihres Tenants gefüllt sein.