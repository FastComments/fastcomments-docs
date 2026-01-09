[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Vi har gennemgået, hvordan `urlId` er id'et for siden eller artiklen, som kommentarerne er tilknyttet.

For at opsummere: hvis ikke defineret, vil `urlId` som standard være den aktuelle sides URL.

Hvad med SPAs, eller Single-Page-Applications, hvor siden eller indholdet, som kommentarerne er tilknyttet, ændrer sig
dynamisk uden en ny sideindlæsning?

#### Angular, React, Vue, osv.

Med vores biblioteker som Angular og React vil en simpel opdatering af `urlId`-egenskaben, der sendes til widget'en,
få kommentarswidget'en til at opdatere. Du kan se dette i praksis for React-appen, for eksempel, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">her</a>.

#### VanillaJS

Hvis du bruger VanillaJS-biblioteket, er det en smule mere kompliceret, da der ikke er et framework som Angular eller React
til at håndtere databinding eller udbredelse af tilstand.

Når du instantierer VanillaJS-widget'en, returnerer den nogle funktioner, der kan kaldes for at opdatere den.

Her er et funktionelt eksempel, hvor vi ændrer side-hashen og opdaterer kommentarswidget'en:

[inline-code-attrs-start title = 'Eksempel på skift af side-hash'; inline-code-attrs-end]
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

            config.url = locationString; // We update url, too, so notifications can link back to the right page
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]