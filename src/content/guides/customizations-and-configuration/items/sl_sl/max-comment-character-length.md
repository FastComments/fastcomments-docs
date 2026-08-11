[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Največje število znakov, ki jih je dovoljeno vnesti v polje za vnos komentarja, je mogoče omejiti s parametrom **maxCommentCharacterLength**.

Privzeta vrednost je 2000.

Stvari, kot so URL-ji slik, niso vključeni v določanje dolžine.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Omeji dolžino komentarja'; code-example-end]

To je mogoče prilagoditi brez kode na strani za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Polje največje velikosti komentarja na strani za prilagajanje gradnika, ki se uporablja za omejitev števila znakov, ki jih lahko komentar vsebuje'; title='Omeji dolžino komentarja' app-screenshot-end]