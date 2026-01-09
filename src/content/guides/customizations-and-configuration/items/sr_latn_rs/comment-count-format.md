[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Broj komentara koji se prikazuje na vrhu widgeta za komentare može se prilagoditi.

Ovo može biti zamenjeno bilo kojim stringom, a vrednost **[count]** biće zamenjena stvarnim brojem, lokalizovanim za korisnika.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]