---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Die maximale Anzahl an Zeichen, die im Kommentar‑Eingabefeld eingegeben werden dürfen, kann durch den Parameter **maxCommentCharacterLength** begrenzt werden.

Der Standardwert ist 2000.

Dinge wie Bild‑URLs werden bei der Längenbestimmung nicht berücksichtigt.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Kommentar-Länge begrenzen'; code-example-end]

Dies kann ohne Code auf der Widget‑Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Maximales Kommentargrößenfeld auf der Widget-Anpassungsseite, das verwendet wird, um die maximale Zeichenanzahl eines Kommentars zu begrenzen'; title='Kommentar-Länge begrenzen' app-screenshot-end]

---