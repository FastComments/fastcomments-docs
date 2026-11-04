[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Wir haben bereits behandelt, wie `urlId` die ID der Seite oder des Artikels ist, mit der die Kommentare verknüpft sind.

Zur Erinnerung: Wenn `urlId` nicht definiert ist, wird standardmäßig die URL der aktuellen Seite verwendet.

Was ist mit SPAs (Single-Page-Applications), bei denen sich die Seite oder der Inhalt, mit dem die Kommentare verknüpft sind, dynamisch ändert, ohne dass die Seite neu geladen wird?

#### Angular, React, Vue, etc.

Mit unseren Bibliotheken wie Angular und React genügt es, die `urlId`-Eigenschaft zu aktualisieren, die an das Widget übergeben wird, damit das Kommentar-Widget neu geladen wird. Sie können das in Aktion sehen, zum Beispiel in der React-App, <a href="https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx#L39" target="_blank">hier</a>.

#### VanillaJS

Wenn Sie die VanillaJS-Bibliothek verwenden, ist es etwas komplizierter, da es kein Framework wie Angular oder React gibt, das das Data Binding oder die State-Propagation übernimmt.

Wenn Sie das VanillaJS-Widget instanziieren, werden einige Funktionen zurückgegeben, die aufgerufen werden können, um es zu aktualisieren.

Hier ist ein funktionales Beispiel, bei dem wir den Seiten-Hash ändern und das Kommentar-Widget aktualisieren:

[inline-code-attrs-start title = 'Beispiel zum Wechseln des Seiten-Hashs'; inline-code-attrs-end]
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

            config.url = locationString; // Wir aktualisieren auch url, damit Benachrichtigungen auf die richtige Seite verlinken können
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]