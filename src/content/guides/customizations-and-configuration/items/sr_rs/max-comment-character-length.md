[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Maksimalni broj znakova koji se smeju uneti u polje za unos komentara može se ograničiti parametrima **maxCommentCharacterLength**.

Podrazumevana vrednost je 2000.

Stvari poput URL‑ova slika se ne uzimaju u obzir pri određivanju dužine.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Ograniči dužinu komentara'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Polje za maksimalnu veličinu komentara na stranici za prilagođavanje widgeta, koristi se za ograničavanje broja znakova koje komentar može da sadrži'; title='Ograniči dužinu komentara' app-screenshot-end]