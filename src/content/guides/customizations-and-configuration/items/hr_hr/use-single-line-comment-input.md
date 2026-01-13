[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments će korisniku dopustiti da unese komentar s onoliko redaka koliko želi, do zadanog ograničenja znakova.

Međutim, može biti poželjno ograničiti korisnika da unese samo jedan redak teksta. Neki primjeri upotrebe uključuju online nadmetanje, ili live chat, za koje se FastComments
može koristiti.

Omogućujemo zastavicu **useSingleLineCommentInput** na sljedeći način:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

To se također može učiniti bez koda. Na stranici za prilagodbu widgeta pogledajte odjeljak "Omogući unos komentara u jednom retku".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Imajte na umu da su komentari na svakoj stranici za svaki smjer sortiranja prethodno izračunati, pa svi smjerovi sortiranja imaju istu izvedbu.