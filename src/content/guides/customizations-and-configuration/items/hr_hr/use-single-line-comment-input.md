[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments će dopustiti korisniku da unese komentar s onoliko redaka koliko želi, do zadnog ograničenja znakova.

Međutim, možda je poželjno ograničiti korisnika da unese samo jedan redak teksta. Neki primjeri upotrebe uključuju online licitiranje ili live chat, za koje se FastComments može koristiti.

Omogućavamo zastavicu **useSingleLineCommentInput** na sljedeći način:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Ovo se također može učiniti bez koda. Na stranici za prilagodbu widgeta, pogledajte odjeljak "Enable Single-Line Comment Input" sekciju.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Potvrdni okvir za unos jednorednog komentara uključen na stranici za prilagodbu widgeta, ograničavajući unos na jedan redak'; title='Omogući unos jednorednog komentara' app-screenshot-end]

Napomena: komentari na svakoj stranici za svaki smjer sortiranja su unaprijed izračunati, pa svi smjerovi sortiranja imaju istu izvedbu.