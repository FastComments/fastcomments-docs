[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Maksimalni broj znakova koji se smiju unijeti u polje za unos komentara može se ograničiti parametrom **maxCommentCharacterLength**.

Zadana vrijednost je 2000.

Stvari poput URL-ova slika nisu uključene u određivanje duljine.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Ograničenje duljine komentara'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Polje za maksimalnu veličinu komentara na stranici za prilagodbu widgeta, koje se koristi za ograničavanje broja znakova koje komentar može sadržavati'; title='Ograničenje duljine komentara' app-screenshot-end]