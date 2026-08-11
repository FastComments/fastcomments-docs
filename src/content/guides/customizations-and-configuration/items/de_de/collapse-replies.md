[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Standardmäßig werden Antworten auf Top‑Level‑Kommentare angezeigt.

Dies kann so konfiguriert werden, dass der Benutzer auf „Show Replies“ bei den Top‑Level‑Kommentaren klicken muss, um die Kind‑Kommentare zu sehen.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Dies kann ohne Code auf der Widget‑Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Option zum Ausblenden von Antworten in der Widget‑Anpassungs‑UI, die Kind‑Kommentare hinter einem „Show Replies“-Link versteckt'; title='Antworten ausblenden' app-screenshot-end]

Diese Einstellung wirkt sich nicht auf die Anzahl der zunächst geladenen Top‑Level‑Kommentare aus. Wenn Sie einen Top‑Level‑Kommentar und 29 Kind‑Kommentare haben, bewirkt diese Einstellung:

- Sie sehen den Top‑Level‑Kommentar.
- Sie sehen Show Replies (29) unter diesem Kommentar.

Wenn Sie in Kombination mit dieser Option alle Top‑Level‑Kommentare anzeigen möchten, setzen Sie [starting page to -1](#starting-page).