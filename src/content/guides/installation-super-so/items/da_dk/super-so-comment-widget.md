## Tilføjelse af en Live-kommentar-widget til dine Super.so Notion-artikler

Ud over Collab Chat kan du tilføje en traditionel kommentar-widget i bunden af dine Notion-artikler. Dette giver læsere mulighed for at efterlade kommentarer og føre diskussioner om hele artiklen.

### Installationstrin

Kopiér følgende kode og indsæt den i `Body`-sektionen i dine Super.so siteindstillinger:

[inline-code-attrs-start title = 'Super.so FastComments Live-kommentar-widget'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Ryd op i eksisterende instans
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Opret nyt mål
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Initialiser FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Opdater nuværende pathname
            currentPathname = window.location.pathname;
        }

        // Første indlæsning
        load();

        // Tjek hver 500 ms for ændringer
        setInterval(() => {
            // Genindlæs hvis pathname ændret
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Genindlæs hvis widget blev fjernet
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Genindlæs hvis container blev tømt
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Vigtige bemærkninger

- Kommentar-widget'en vises i bunden af dine Notion-artikler
- Hver side får sin egen unikke kommentartråd baseret på URL-stien
- Sørg for at erstatte "demo" med dit faktiske tenant ID fra din FastComments-konto
- Widget'en håndterer automatisk Super.so's dynamiske sideindlæsning