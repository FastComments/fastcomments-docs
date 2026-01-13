[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

We hebben uitgelegd hoe `urlId` het pagina- of artikel-id is waaraan de reacties gekoppeld zijn.

Ter herinnering: als deze niet is gedefinieerd, zal `urlId` standaard de URL van de huidige pagina zijn.

Hoe zit het met SPA's, of Single-Page Applications, waarbij de pagina of inhoud waaraan de reacties zijn gekoppeld dynamisch verandert zonder de pagina opnieuw te laden?

#### Angular, React, Vue, enz.

Met onze bibliotheken zoals Angular en React zal het bijwerken van de `urlId`-eigenschap die aan de widget wordt doorgegeven ervoor zorgen dat de reactie-widget wordt vernieuwd. Je kunt dit in actie zien voor de React-app, bijvoorbeeld <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">hier</a>.

#### VanillaJS

Als je de VanillaJS-bibliotheek gebruikt, is het iets ingewikkelder omdat er geen framework zoals Angular of React is om de datakoppeling of toestandspropagatie af te handelen.

Wanneer je de VanillaJS-widget instancieert, geeft deze enkele functies terug die kunnen worden aangeroepen om hem bij te werken.

Hier is een functioneel voorbeeld waarin we de pagina-hash wijzigen en de reactie-widget bijwerken:

[inline-code-attrs-start title = 'Voorbeeld: Pagina-hash wijzigen'; inline-code-attrs-end]
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

            config.url = locationString; // We werken de url ook bij, zodat meldingen naar de juiste pagina kunnen linken
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]