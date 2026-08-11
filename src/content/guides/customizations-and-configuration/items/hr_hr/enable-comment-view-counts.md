[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments ne prati tko je pregledao svaki komentar niti pruža bilo kakve statistike o tome.

Međutim, možemo omogućiti ovu značajku, a sustav će početi pratiti kako svaki korisnik pomiče do komentara.

Kada se to dogodi, broj pored ikone oka prikazane na svakom komentaru će se povećati. Broj se ažurira u stvarnom vremenu i skraćuje prema lokalizaciji korisnika.

Možemo to omogućiti postavljanjem **enableViewCounts** zastavice na true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Omogućavanje broja pregleda komentara'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Stranica za prilagodbu widgeta s označenim potvrdnim okvirom za prikaz broja pregleda, tako da svaki komentar prikazuje ikonu oka i broj'; title='Omogućavanje broja pregleda komentara' app-screenshot-end]

Bilježimo ID korisnika* koji je pregledao komentar, tako da ako ponovno pregledate komentar, broj se ne povećava. Ako pregledate komentar ponovno
nakon dvije godine, broj će se povećati više.

- *Napomena: ili anonimni ID sesije, ili IP korisnika kao hashirana vrijednost.