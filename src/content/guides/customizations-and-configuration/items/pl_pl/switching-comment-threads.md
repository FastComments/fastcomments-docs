[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Omówiliśmy, jak `urlId` jest identyfikatorem strony lub artykułu, z którym powiązane są komentarze.

Aby przypomnieć: jeśli nie zostanie zdefiniowany, `urlId` domyślnie przyjmie wartość bieżącego adresu URL strony.

A co z aplikacjami SPA, czyli Single-Page-Applications, gdzie strona lub zawartość, do której przypisane są komentarze,
zmienia się dynamicznie bez pełnego przeładowania strony?

#### Angular, React, Vue itp.

W naszych bibliotekach, takich jak Angular i React, wystarczy zaktualizować właściwość `urlId` przekazywaną do widżetu,
aby widżet komentarzy odświeżył się. Możesz zobaczyć to w akcji dla aplikacji React, na przykład <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">tutaj</a>.

#### VanillaJS

Jeśli używasz biblioteki VanillaJS, jest to nieco bardziej skomplikowane, ponieważ nie ma tam frameworka takiego jak Angular czy React,
który obsługiwałby wiązanie danych lub propagację stanu.

Kiedy tworzysz instancję widżetu VanillaJS, zwraca on kilka funkcji, które można wywołać, aby go zaktualizować.

Oto działający przykład, w którym zmieniamy fragment strony (hash) i aktualizujemy widżet komentarzy:

[inline-code-attrs-start title = 'Przykład zmiany fragmentu (hash) strony'; inline-code-attrs-end]
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