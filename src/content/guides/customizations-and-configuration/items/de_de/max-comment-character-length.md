---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Die maximale Anzahl von Zeichen, die im Kommentarfeld eingegeben werden dürfen, kann durch den Parameter **maxCommentCharacterLength** begrenzt werden.

Der Standardwert beträgt 2000.

Elemente wie Bild-URLs werden nicht in die Längenberechnung einbezogen.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Dies kann ohne Code auf der Widget-Anpassungsseite konfiguriert werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---