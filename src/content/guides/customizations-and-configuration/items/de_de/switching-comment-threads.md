[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Wir haben behandelt, wie `urlId` die Seiten- oder Artikel-ID ist, an die die Kommentare gebunden sind.

Außerdem, zur Erinnerung: wenn nicht definiert, wird die `urlId` standardmäßig auf die aktuelle Seiten-URL gesetzt.

Wie sieht es mit SPAs, oder Single-Page-Applications, aus, bei denen die Seite oder der Inhalt, an den die Kommentare gebunden sind, sich
dynamisch ändert, ohne dass die Seite neu geladen wird?

#### Angular, React, Vue, etc

Mit unseren Bibliotheken wie Angular und React bewirkt das einfache Aktualisieren der an das Widget übergebenen `urlId`-Eigenschaft,
dass das Kommentar-Widget aktualisiert wird. Sie können dies beispielsweise in der React-App <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">hier</a> sehen.

#### VanillaJS

Wenn Sie die VanillaJS-Bibliothek verwenden, ist es etwas komplizierter, da es kein Framework wie Angular oder React gibt,
das die Datenbindung oder Zustandsweitergabe übernimmt.

Wenn Sie das VanillaJS-Widget instanziieren, gibt es einige Funktionen zurück, die aufgerufen werden können, um es zu aktualisieren.

Hier ein funktionales Beispiel, in dem wir den Seiten-Hash ändern und das Kommentar-Widget aktualisieren:

[inline-code-attrs-start title = 'Beispiel zum Wechseln des Seiten-Hashes'; inline-code-attrs-end]
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