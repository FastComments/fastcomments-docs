[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Obradili smo kako je `urlId` ID stranice ili članka uz koji su komentari vezani.

Također, za podsjetnik, ako nije definirano, `urlId` će se zadano postaviti na URL trenutne stranice.

A što je s SPA-ovima, odnosno Single-Page Applicationima, gdje se stranica ili sadržaj kojem su komentari vezani mijenja dinamički bez ponovnog učitavanja stranice?

#### Angular, React, Vue, itd

S našim bibliotekama poput Angulara i Reacta, jednostavnim ažuriranjem svojstva `urlId` koje se prosljeđuje widgetu, widget za komentare će se osvježiti. Ovo možete vidjeti u praksi za React aplikaciju, na primjer, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">ovdje</a>.

#### VanillaJS

Ako koristite VanillaJS biblioteku, stvar je nešto složenija jer ne postoji okvir poput Angulara ili Reacta koji bi rukovao vezivanjem podataka ili propagacijom stanja.

Kada instancirate VanillaJS widget, on vraća neke funkcije koje se mogu pozvati za njegovo ažuriranje.

Evo funkcionalnog primjera u kojem mijenjamo hash stranice i ažuriramo widget za komentare:

[inline-code-attrs-start title = 'Primjer promjene hash-a stranice'; inline-code-attrs-end]
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

            config.url = locationString; // Također ažuriramo url, kako bi obavijesti mogle voditi natrag na pravu stranicu
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---