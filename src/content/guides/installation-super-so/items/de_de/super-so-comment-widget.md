---
## Hinzufügen eines Live-Kommentar-Widgets zu Ihren Super.so Notion-Artikeln

Zusätzlich zum Collab Chat können Sie am Ende Ihrer Notion-Artikel ein klassisches Kommentar-Widget hinzufügen. Dadurch können Leser Kommentare hinterlassen und Diskussionen über den gesamten Artikel führen.

### Installationsschritte

Kopieren Sie den folgenden Code und fügen Sie ihn in den `Body`-Abschnitt Ihrer Super.so-Site-Einstellungen ein:

[inline-code-attrs-start title = 'Super.so FastComments Live-Kommentar-Widget'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Vorhandene Instanz bereinigen
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Neues Ziel erstellen
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // FastComments initialisieren
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Aktuellen Pfadnamen aktualisieren
            currentPathname = window.location.pathname;
        }

        // Initiales Laden
        load();

        // Alle 500 ms auf Änderungen prüfen
        setInterval(() => {
            // Neu laden, wenn sich der Pfadname geändert hat
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
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Wichtige Hinweise

- Das Kommentar-Widget wird am Ende Ihrer Notion-Artikel angezeigt
- Jede Seite erhält einen eigenen, eindeutigen Kommentar-Thread basierend auf dem URL-Pfad
- Stellen Sie sicher, dass Sie "demo" durch Ihre tatsächliche Tenant-ID aus Ihrem FastComments-Konto ersetzen
- Das Widget verarbeitet automatisch das dynamische Laden von Seiten bei Super.so

---