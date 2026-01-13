[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Objasnili smo kako je `urlId` identifikator stranice ili članka na koji su komentari vezani.

Takođe, da rezimiramo, ako nije definisan, `urlId` će podrazumevano biti URL trenutne stranice.

Šta je sa SPA-ima, odnosno Single-Page-Applications, gde se stranica ili sadržaj na koji su komentari vezani menja dinamički bez ponovnog učitavanja cele stranice?

#### Angular, React, Vue, i dr.

Sa našim bibliotekama kao što su Angular i React, jednostavno ažuriranje `urlId` svojstva prosleđenog widgetu prouzrokovaće osvežavanje widgeta za komentare. To možete videti u praksi za React aplikaciju, na primer, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">ovde</a>.

#### VanillaJS

Ako koristite VanillaJS biblioteku, to je malo složenije jer nema framework-a poput Angular-a ili React-a koji bi rukovao vezivanjem podataka ili propagacijom stanja.

Kada instancirate VanillaJS widget, on vraća neke funkcije koje se mogu pozvati da ga ažuriraju.

Evo funkcionalnog primera gde menjamo hash stranice i ažuriramo widget za komentare:

[inline-code-attrs-start title = 'Primer promene hash-a stranice'; inline-code-attrs-end]
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

            config.url = locationString; // Takođe ažuriramo url, kako bi obaveštenja mogla da vode nazad na pravu stranicu
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]