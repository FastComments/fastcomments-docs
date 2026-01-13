[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Standardmäßig ist Live-Kommentierung aktiviert. Das bedeutet, dass wenn Kommentare hinzugefügt, gelöscht, bearbeitet oder angepinnt werden, die Änderungen allen Nutzern, die denselben Kommentar-Thread ansehen, gleichzeitig angezeigt werden sollten.

Standardmäßig erscheinen diese neuen Kommentare jedoch unter einer dynamisch eingeblendeten Schaltfläche mit einem Text ähnlich wie „Zeige 2 neue Kommentare“.

Wenn die neuen Kommentare direkte Antworten auf die Seite sind, wird die Schaltfläche oben im Kommentar-Thread angezeigt. Wenn sie Antworten auf einen bestimmten Kommentar sind, wird die Schaltfläche unter diesem Kommentar angezeigt.

Dies verhindert, dass sich die Seitengröße ständig für den Nutzer ändert, was beim Versuch, die Bildlaufleiste zu greifen, zu Frustration führen kann.

Für einige Anwendungsfälle, wie Live-Auktionen oder Online-Events, ist dieses Verhalten nicht erwünscht – hier möchte man möglicherweise, dass das Kommentar-Widget eher wie ein „Chat“-Feld funktioniert, in dem neue Kommentare „sofort angezeigt werden“.

Daher der Name der Flag, die diese Funktion aktiviert: **showLiveRightAway**.

Wir können es wie folgt aktivieren:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Dies kann ohne Code auf der Seite zur Widget-Anpassung angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---