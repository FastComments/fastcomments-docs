## Dodavanje vidžeta komentara uživo na vaše Super.so Notion članke

Pored Collab Chat, možete dodati tradicionalni vidžet komentara na dno vaših Notion članaka. Ovo omogućava čitaocima da ostavljaju komentare i vode diskusije o cijelom članku.

### Koraci instalacije

Kopirajte sledeći kod i zalijepite ga u sekciju `Body` u podešavanjima vašeg Super.so sajta:

[inline-code-attrs-start title = 'Super.so FastComments vidžet komentara uživo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Clean up existing instance
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Create new target
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Initialize FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Update current pathname
            currentPathname = window.location.pathname;
        }

        // Initial load
        load();

        // Check every 500ms for changes
        setInterval(() => {
            // Reload if pathname changed
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Reload if widget was removed
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Reload if container was emptied
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Važne napomene

- Vidžet komentara će se pojaviti na dnu vaših Notion članaka
- Svaka stranica dobija vlastitu jedinstvenu nit komentara zasnovanu na putanji URL-a
- Obavezno zamijenite `"demo"` stvarnim tenant ID-jem iz vašeg FastComments naloga
- Vidžet automatski upravlja dinamičkim učitavanjem stranica Super.so

---