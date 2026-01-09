## Dodajanje vtičnika za komentarje v živo v vaše Super.so Notion članke

Poleg Collab Chata lahko na dno vaših Notion člankov dodate tradicionalni vtičnik za komentarje. To bralcem omogoča, da pustijo komentarje in vodijo razprave o celotnem članku.

### Koraki namestitve

Kopirajte naslednjo kodo in jo prilepite v razdelek `Body` v nastavitvah vaše spletne strani Super.so:

[inline-code-attrs-start title = 'FastComments vtičnik za komentarje v živo za Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Počisti obstoječo instanco
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Ustvari nov cilj
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Inicializiraj FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Posodobi trenutni pathname
            currentPathname = window.location.pathname;
        }

        // Začetno nalaganje
        load();

        // Preverjaj spremembe vsake 500 ms
        setInterval(() => {
            // Ponovno naloži, če se je pathname spremenil
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Ponovno naloži, če je bil vtičnik odstranjen
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Ponovno naloži, če je bila vsebina kontejnerja izpraznjena
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Pomembne opombe

- Vtičnik za komentarje se bo prikazal na dnu vaših Notion člankov
- Vsaka stran dobi svojo edinstveno nit komentarjev, ki temelji na poti URL
- Poskrbite, da boste zamenjali `"demo"` z dejanskim ID-jem najemnika iz vašega FastComments računa
- Vtičnik samodejno upravlja dinamično nalaganje strani Super.so

---