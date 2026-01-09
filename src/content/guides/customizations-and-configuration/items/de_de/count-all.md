[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Die im oberen Bereich des Kommentar-Widgets angezeigte Kommentaranzahl kann entweder alle "top-level" Kommentare anzeigen, das heißt jene Antworten, die
Antworten direkt auf die Seite oder den Artikel selbst sind, oder es kann eine Zählung von **allen** verschachtelten Kommentaren sein.

Standardmäßig ist dies `true` - es ist eine Zählung von Letzterem - allen Kommentaren. In älteren Versionen des Kommentar-Widgets ist der
Standardwert `false`.

Wir können das Verhalten ändern, sodass es eine Zählung von **allen** verschachtelten Kommentaren ist, indem wir das **countAll** Flag auf true setzen.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Wenn wir möchten, dass die Zählung nur die Top-Level-Kommentare widerspiegelt, setzen wir das Flag auf false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Dies kann derzeit nicht ohne Codeänderungen angepasst werden.