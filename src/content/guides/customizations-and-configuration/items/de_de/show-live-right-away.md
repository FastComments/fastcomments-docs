[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Standardmäßig ist Live‑Kommentierung aktiviert. Das bedeutet, dass wenn Kommentare hinzugefügt, gelöscht, bearbeitet oder angeheftet werden, die Änderungen allen Benutzern, die den Kommentar‑Thread gleichzeitig ansehen, angezeigt werden.

Allerdings werden diese neuen Kommentare standardmäßig unter einem dynamisch angezeigten Button mit einem Text ähnlich wie „Show 2 New Comments“ angezeigt.

Wenn die neuen Kommentare direkte Antworten auf die Seite sind, wird der Button oben im Kommentar‑Thread angezeigt. Wenn sie Antworten auf einen bestimmten Kommentar sind, wird der Button unter diesem Kommentar angezeigt.

Dies soll verhindern, dass die Seitengröße für den Benutzer ständig wechselt, was beim Versuch, die Bildlaufleiste zu greifen, zu Frustration führen kann.

Für einige Anwendungsfälle, wie Live‑Auktionen oder Online‑Events, ist dieses Verhalten nicht erwünscht – Sie möchten möglicherweise, dass das Kommentar‑Widget eher wie ein „Chat“-Fenster funktioniert, bei dem neue Kommentare sofort angezeigt werden.

Daher heißt das Flag, das diese Funktion aktiviert: **showLiveRightAway**.

Wir können es wie folgt aktivieren:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Live-Kommentare sofort anzeigen'; code-example-end]

Dies kann ohne Code auf der Widget‑Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Einstellung zum Ausblenden von Live-Kommentaren umgeschaltet, sodass neue Kommentare sofort erscheinen statt hinter einem Button'; title='Live-Kommentare sofort anzeigen' app-screenshot-end]