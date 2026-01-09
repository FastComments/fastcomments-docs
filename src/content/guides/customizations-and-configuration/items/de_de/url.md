[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Beim Versenden von Benachrichtigungs-E-Mails oder beim Anzeigen von Kommentaren in Benutzeroberflächen wie der Moderationsseite ist es hilfreich, einen Link
vom Kommentar zur Seite zu setzen, auf der er sich befindet.

Wenn die URL ID nicht immer eine ID ist, müssen wir die URL an einer anderen Stelle speichern. Genau dafür ist die "url" property gedacht, die wie folgt definiert ist.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Ein häufiger Anwendungsfall ist, den Kommentar-Thread an einen Bezeichner wie einen Artikel zu binden und dann beispielsweise auf eine bestimmte Seite zurückzuverlinken:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

Die URL wird nicht von üblichen Marketing-Parametern bereinigt. Standardmäßig wird genau die aktuelle Seiten-URL mit dem Kommentar gespeichert.