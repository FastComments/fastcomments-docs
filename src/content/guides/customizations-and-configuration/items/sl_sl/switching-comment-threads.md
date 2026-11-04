[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Pokrili smo, kako je `urlId` id strani ali članka, na katerega so vezani komentarji.

Pravtako, za povzetek: če ni definiran, se `urlId` privzeto nastavi na URL trenutne strani.

Kaj pa SPA-ji (Single-Page-Applications), kjer se stran ali vsebina, na katero so vezani komentarji, dinamično spreminja brez ponovnega nalaganja strani?

#### Angular, React, Vue itd.

Z našimi knjižnicami, kot sta Angular in React, preprosta posodobitev lastnosti `urlId`, podane gradniku, povzroči osvežitev gradnika komentarjev. To lahko vidite v akciji za React aplikacijo na primer <a href="https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx#L39" target="_blank">tukaj</a>.

#### VanillaJS

Če uporabljate knjižnico VanillaJS, je to nekoliko bolj zapleteno, saj ni ogrodja, kot sta Angular ali React, ki bi upravljalo vezavo podatkov ali propagacijo stanja.

Ko instancirate VanillaJS gradnik, ta vrne nekatere funkcije, ki jih lahko pokličete za njegovo posodobitev.

Tu je funkcionalen primer, kjer spremenimo hash strani in posodobimo gradnik komentarjev:

[inline-code-attrs-start title = 'Primer preklopa hash strani'; inline-code-attrs-end]
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

            config.url = locationString; // Posodobimo tudi url, da se lahko obvestila povežejo nazaj na pravo stran
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]