## Dodavanje vidžeta za komentare uživo u vaše Super.so Notion članke

Pored Collab Chata, možete dodati tradicionalni vidžet za komentare na dno vaših Notion članaka. Ovo omogućava čitaocima da ostavljaju komentare i vode diskusije o celom članku.

### Koraci instalacije

Kopirajte sledeći kod i nalepite ga u odeljak `Body` podešavanja vaše Super.so stranice:

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

            // Uklanjanje postojeće instance
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Kreiraj novi target
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Inicijalizuj FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Ažuriraj trenutni pathname
            currentPathname = window.location.pathname;
        }

        // Početno učitavanje
        load();

        // Proveravaj promene svakih 500ms
        setInterval(() => {
            // Ponovo učitaj ako se pathname promenio
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Ponovo učitaj ako je vidžet uklonjen
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Ponovo učitaj ako je kontejner očišćen
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

- Vidžet za komentare pojaviće se na dnu vaših Notion članaka
- Svaka stranica dobija sopstvenu jedinstvenu nit komentara zasnovanu na putanji URL-a
- Obavezno zamenite "demo" vašim stvarnim tenant ID-jem iz vašeg FastComments naloga
- Vidžet automatski podržava dinamičko učitavanje stranica na Super.so