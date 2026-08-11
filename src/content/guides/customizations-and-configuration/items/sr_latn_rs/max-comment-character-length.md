[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Maksimalni broj znakova koji se mogu uneti u polje za unos komentara može biti ograničen parametrima **maxCommentCharacterLength**.

Podrazumevana vrednost je 2000.

Stvari poput URL‑ova slika nisu uključene u određivanje dužine.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Ograničenje dužine komentara'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Polje za maksimalnu veličinu komentara na stranici za prilagođavanje widgeta, koje se koristi da ograniči koliko znakova komentar može da sadrži'; title='Ograničenje dužine komentara' app-screenshot-end]