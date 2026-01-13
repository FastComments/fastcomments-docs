---
[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Standardmäßig erlaubt FastComments dem Benutzer, einen Kommentar mit so vielen Zeilen einzugeben, wie er möchte, bis zum standardmäßigen Zeichenlimit.

Es kann jedoch wünschenswert sein, den Benutzer darauf zu beschränken, nur eine einzige Textzeile einzugeben. Beispiele für Anwendungsfälle sind Online-Auktionen oder Live-Chat, für die FastComments
verwendet werden kann.

Wir aktivieren das Flag **useSingleLineCommentInput** wie folgt:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Dies kann auch ohne Code erfolgen. Auf der Seite zur Anpassung des Widgets finden Sie den Abschnitt "Enable Single-Line Comment Input".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Beachten Sie, dass die Kommentare auf jeder Seite für jede Sortierrichtung vorab berechnet werden, sodass alle Sortierrichtungen die gleiche Leistung haben.

---