[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Broj komentara prikazan na vrhu widgeta za komentare može se prilagoditi.

Ovo se može zameniti bilo kojim stringom, a vrednost **[count]** će biti zamenjena vrednošću broja, lokalizovanom za korisnika.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Prilagođavanje teksta broja komentara'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Polje za tekst broja komentara na stranici za prilagođavanje widgeta, gde se [count] zamenjuje trenutnim ukupnim brojem'; title='Prilagođavanje teksta broja komentara' app-screenshot-end]