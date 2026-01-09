[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Abbiamo spiegato come `urlId` sia l'id della pagina o dell'articolo a cui sono associati i commenti.

Inoltre, per ricapitolare, se non è definito il `urlId` verrà impostato di default sull'URL della pagina corrente.

Che succede con le SPA, o applicazioni a pagina singola, dove la pagina o il contenuto a cui sono associati i commenti cambia dinamicamente senza un ricaricamento della pagina?

#### Angular, React, Vue, ecc.

Con le nostre librerie come Angular e React, aggiornare semplicemente la proprietà `urlId` passata al widget farà sì che il widget dei commenti si aggiorni. Puoi vedere questo in azione per l'app React, per esempio, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">qui</a>.

#### VanillaJS

Se stai usando la libreria VanillaJS è un po' più complicato poiché non c'è un framework come Angular o React che gestisca il data binding o la propagazione dello stato.

Quando istanzi il widget VanillaJS, questo restituisce alcune funzioni che possono essere chiamate per aggiornarlo.

Ecco un esempio funzionale in cui cambiamo l'hash della pagina e aggiorniamo il widget dei commenti:

[inline-code-attrs-start title = 'Esempio di cambio dell\'hash della pagina'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // Aggiorniamo anche url, così le notifiche possono collegarsi alla pagina corretta
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---