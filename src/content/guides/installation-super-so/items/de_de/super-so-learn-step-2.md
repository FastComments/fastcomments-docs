In diesem nächsten Schritt musst du den unten vorbereiteten Widget-Code kopieren.

Solange du bei FastComments.com eingeloggt bist, enthält der folgende Code-Schnipsel bereits deine Kontoinformationen. Kopieren wir ihn:

[inline-code-attrs-start title = 'Super.so FastComments Collab Chat Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Bestehende Instanz aufräumen
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Bestehende Top-Bar aufräumen, falls vorhanden
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Neue Top-Bar erstellen
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

        // Erstes Laden
        load();

        // Alle 500ms auf Änderungen prüfen
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

Füge nun den Code in den Bereich `Body` ein:

<div class="screenshot white-bg">
    <div class="title">Eingefügter Code</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Eingefügter Code" />
</div>

Wenn du nach dem Einfügen des Codes eine „this is a demo message"-Meldung siehst:

- Stelle sicher, dass du in deinem fastcomments.com-Konto eingeloggt bist.
- Stelle sicher, dass Cookies von Drittanbietern aktiviert sind.
- Lade dann diese Seite neu und kopiere den Code-Schnipsel erneut. Er sollte mit der Kennung deines Tenants als `tenantId` gefüllt sein.