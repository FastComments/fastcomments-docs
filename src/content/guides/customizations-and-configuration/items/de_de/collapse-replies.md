[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Standardmäßig werden Antworten auf Top-Level-Kommentare angezeigt.

Dies kann so konfiguriert werden, dass der Nutzer auf den Top-Level-Kommentaren auf "Antworten anzeigen" klicken muss, um die untergeordneten Kommentare zu sehen.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Dies kann ohne Code auf der Seite zur Anpassung des Widgets konfiguriert werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Diese Einstellung wirkt sich nicht auf die Anzahl der anfänglich geladenen Top-Level-Kommentare aus. Wenn Sie einen Top-Level-Kommentar und 29 Unterkommentare haben, sehen Sie bei aktivierter Einstellung:

- Sehen Sie den Top-Level-Kommentar.
- Unter diesem Kommentar sehen Sie "Antworten anzeigen (29)".

Wenn Sie alle Top-Level-Kommentare in Kombination mit dieser Option anzeigen möchten, setzen Sie [Anfangsseite auf -1](#starting-page).