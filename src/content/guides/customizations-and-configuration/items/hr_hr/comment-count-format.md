[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Broj komentara prikazan na vrhu widgeta za komentare može se prilagoditi.

Ovo se može zamijeniti bilo kojim nizom, a vrijednost **[count]** bit će zamijenjena s vrijednošću broja, lokaliziranom za korisnika.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Prilagođavanje teksta broja komentara'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Polje teksta broja komentara na stranici za prilagodbu widgeta, gdje se [count] zamjenjuje s trenutnim ukupnim brojem'; title='Prilagođavanje teksta broja komentara' app-screenshot-end]