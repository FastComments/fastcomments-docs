[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments omogućava korisniku da unese komentar sa koliko god linija želi, do podrazumevanog ograničenja znakova.

Međutim, može biti poželjno ograničiti korisnika da unese samo jednu liniju teksta. Neki primeri upotrebe uključuju online licitacije ili live chat, za koje se FastComments može koristiti.

Omogućavamo zastavicu **useSingleLineCommentInput** na sledeći način:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak „Enable Single-Line Comment Input“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Polje za unos jedne linije komentara je uključeno na stranici za prilagođavanje widgeta, ograničavajući unos na jednu liniju'; title='Omogući unos jedne linije komentara' app-screenshot-end]

Napomena: komentari na svakoj stranici za svaki smer sortiranja su unapred izračunati, tako da svi smerovi sortiranja imaju istu performansu.