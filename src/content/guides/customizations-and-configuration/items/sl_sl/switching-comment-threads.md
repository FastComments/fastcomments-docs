[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Opisali smo, kako je `urlId` identifikator strani ali članka, s katerim so komentarji povezani.

Poleg tega, če povzamemo: če ni določen, bo `urlId` privzeto URL trenutne strani.

Kaj pa SPAs oziroma Single-Page Applications, kjer se stran ali vsebina, s katero so povezani komentarji, spreminja dinamično brez ponovnega nalaganja strani?

#### Angular, React, Vue, etc

Z našimi knjižnicami, kot sta Angular in React, bo preprosta posodobitev lastnosti `urlId`, posredovane pripomočku, povzročila osvežitev komentarnega pripomočka. To si lahko ogledate v delovanju za aplikacijo React, na primer <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">tukaj</a>.

#### VanillaJS

Če uporabljate knjižnico VanillaJS, je nekoliko bolj zapleteno, saj ni ogrodja, kot sta Angular ali React, ki bi upravljalo vezanje podatkov ali propagacijo stanja.

Ko ustvarite instanco VanillaJS pripomočka, ta vrne nekaj funkcij, ki jih je mogoče poklicati za njegovo posodobitev.

Tukaj je funkcionalen primer, kjer spremenimo hash strani in posodobimo komentarni pripomoček:

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

            config.url = locationString; // We update url, too, so notifications can link back to the right page
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---