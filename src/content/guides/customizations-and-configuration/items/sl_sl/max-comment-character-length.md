[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Največje število znakov, ki jih je dovoljeno vnesti v polje za vnos komentarja, je mogoče omejiti s parametrom **maxCommentCharacterLength**.

Privzeta vrednost je 2000.

Elementi, kot so URL-ji slik, niso vključeni pri določanju dolžine.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

To lahko prilagodite brez uporabe kode na strani za prilagajanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---