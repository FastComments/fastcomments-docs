## Dodavanje widgeta za komentare uživo na vaše Super.so Notion članke

Pored Collab Chata, možete dodati tradicionalni widget za komentare na dno vaših Notion članaka. Ovo omogućava čitaocima da ostave komentare i vode diskusije o čitavom članku.

### Koraci instalacije

Kopirajte sljedeći kod i zalijepite ga u odjeljak `Body` postavki vaše Super.so stranice:

[inline-code-attrs-start title = 'Super.so FastComments widget za komentare uživo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Inicijalizuj FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Ažuriraj trenutnu putanju
            currentPathname = window.location.pathname;
        }

        // Početno učitavanje
        load();

        // Provjeri svake 500ms ima li promjena
        setInterval(() => {
            // Ponovo učitaj ako se putanja promijenila
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Ponovo učitaj ako je widget uklonjen
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Ponovo učitaj ako je kontejner ispraznjen
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

- Widget za komentare će se pojaviti na dnu vaših Notion članaka
- Svaka stranica dobija svoju jedinstvenu nit komentara na osnovu URL putanje
- Obavezno zamijenite `"demo"` sa stvarnim tenant ID-jem iz vašeg FastComments naloga
- Widget automatski podržava dinamičko učitavanje stranica Super.so