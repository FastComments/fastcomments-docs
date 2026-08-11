[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Standardmäßig erlaubt FastComments dem Benutzer, einen Kommentar mit beliebig vielen Zeilen einzugeben, bis zum standardmäßigen Zeichenlimit.

Es kann jedoch wünschenswert sein, den Benutzer auf die Eingabe einer einzigen Textzeile zu beschränken. Beispiele für Anwendungsfälle sind Online-Auktionen oder Live-Chat, für die FastComments verwendet werden kann.

Wir aktivieren die **useSingleLineCommentInput**-Flagge wie folgt:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Dies kann auch ohne Code durchgeführt werden. Auf der Widget-Anpassungsseite finden Sie den Abschnitt „Enable Single-Line Comment Input“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Einzelzeilen-Kommentar-Eingabekontrollkästchen in der Widget-Anpassungsseite aktiviert, wodurch die Eingabe auf eine Zeile beschränkt wird'; title='Einzelzeilen-Kommentar-Eingabe aktivieren' app-screenshot-end]

Beachten Sie, dass die Kommentare auf jeder Seite für jede Sortierrichtung vorab berechnet werden, sodass alle Sortierrichtungen die gleiche Leistung haben.