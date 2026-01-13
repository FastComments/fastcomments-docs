## Dodavanje vidžeta za komentare uživo u vaše Super.so Notion članke

Uz Collab Chat, možete dodati tradicionalni vidžet za komentare na dno vaših Notion članaka. To omogućuje čitateljima da ostave komentare i vode rasprave o cijelom članku.

### Koraci instalacije

Kopirajte sljedeći kod i zalijepite ga u odjeljak `Body` postavki vaše Super.so stranice:

[inline-code-attrs-start title = 'Super.so FastComments vidžet za komentare uživo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Očisti postojeću instancu
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Kreiraj novi ciljni element
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Inicijaliziraj FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Ažuriraj trenutnu putanju
            currentPathname = window.location.pathname;
        }

        // Početno učitavanje
        load();

        // Provjeri svakih 500 ms za promjene
        setInterval(() => {
            // Ponovno učitaj ako se putanja promijenila
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Ponovno učitaj ako je vidžet uklonjen
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Ponovno učitaj ako je kontejner ispraznjen
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

- Vidžet za komentare pojavit će se na dnu vaših Notion članaka
- Svaka stranica dobiva vlastitu jedinstvenu nit komentara zasnovanu na putanji URL-a
- Pobrinite se zamijeniti `"demo"` svojim stvarnim tenant ID-jem iz FastComments računa
- Vidžet automatski podržava dinamičko učitavanje stranica Super.so-a

---