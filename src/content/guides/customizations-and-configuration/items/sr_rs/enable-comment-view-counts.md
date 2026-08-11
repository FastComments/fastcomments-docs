[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments ne prati ko je pogledao svaki komentar niti pruža bilo kakve statistike u vezi s tim.

Međutim, možemo omogućiti ovu funkciju, i tada će sistem početi da prati kako svaki korisnik skroluje do komentara.

Kada se to desi, broj pored ikone oka prikazane na svakom komentaru će se povećati. Broj se ažurira u realnom vremenu i skraćuje prema lokalnom podešavanju korisnika.

Možemo ovo omogućiti postavljanjem zastavice **enableViewCounts** na true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Omogućavanje broja pregleda komentara'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Stranica za prilagođavanje widgeta sa označenim poljem za broj pregleda tako da svaki komentar prikazuje ikonu oka i broj'; title='Omogućavanje broja pregleda komentara' app-screenshot-end]

Pratimo ID korisnika* koji je pogledao komentar, tako da se pri ponovnom gledanju komentara ne povećava. Ako pogledate komentar ponovo nakon dve godine, broj će se povećati više.

- *Napomena: ili anonimus ID sesije, ili IP korisnika kao heširana vrednost.