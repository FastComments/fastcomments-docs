[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments ne prati ko je pregledao svaki komentar i ne pruža statistiku o tome.

Međutim, ovu funkciju možemo omogućiti, i tada će sistem početi da prati kada svaki korisnik skroluje do komentara.

Kada se to desi, broj pored ikone oka koji se prikazuje na svakom komentaru biće uvećan. Brojač se ažurira uživo i skraćuje se prema lokalnim podešavanjima korisnika.

Ovo možemo omogućiti podešavanjem zastavice **enableViewCounts** na true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Pratimo ID korisnika* koji je pregledao komentar, tako da ako komentar pogledate ponovo broj se neće uvećati. Ako komentar pogledate ponovo nakon dve godine, broj će se ponovo povećati.

- *Napomena: ili anon session id, ili IP korisnika kao hashovana vrednost.