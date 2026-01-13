## Een Live Comment-widget toevoegen aan uw Super.so Notion-artikelen

In aanvulling op Collab Chat kunt u een traditionele commentaarwidget toevoegen aan de onderkant van uw Notion-artikelen. Dit stelt lezers in staat om opmerkingen achter te laten en discussies te voeren over het hele artikel.

### Installatiestappen

Kopieer de volgende code en plak deze in de `Body`-sectie van uw Super.so site-instellingen:

[inline-code-attrs-start title = 'Super.so FastComments Live Comment-widget'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Bestaande instantie opruimen
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Nieuwe target maken
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // FastComments initialiseren
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Huidige pathname bijwerken
            currentPathname = window.location.pathname;
        }

        // Initiële laadactie
        load();

        // Elke 500ms controleren op wijzigingen
        setInterval(() => {
            // Opnieuw laden als pathname is veranderd
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Opnieuw laden als widget is verwijderd
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Opnieuw laden als container geleegd is
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Belangrijke opmerkingen

- De commentaarwidget verschijnt onderaan uw Notion-artikelen
- Elke pagina krijgt een eigen unieke discussiedraad op basis van het URL-pad
- Zorg ervoor dat u `"demo"` vervangt door uw echte tenant-ID uit uw FastComments-account
- De widget handelt automatisch de dynamische paginalading van Super.so af

---