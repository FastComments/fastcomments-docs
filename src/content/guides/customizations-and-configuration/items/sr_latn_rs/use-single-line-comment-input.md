[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments će dozvoliti korisniku da unese komentar sa onoliko redova koliko želi, do podrazumevanog ograničenja broja karaktera.

Međutim, može biti poželjno ograničiti korisnika da unese samo jednu liniju teksta. Neki primeri upotrebe uključuju online licitiranje, ili chat uživo, za koje FastComments
može da se koristi.

Uključujemo zastavicu **useSingleLineCommentInput** na sledeći način:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Ovo se takođe može uraditi i bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak "Omogući unos komentara u jednoj liniji".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Imajte na umu da su komentari na svakoj stranici za svaki smer sortiranja prethodno izračunati, tako da svi smerovi sortiranja imaju iste performanse.