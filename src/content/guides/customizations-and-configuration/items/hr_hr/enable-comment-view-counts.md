---
[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments ne bilježi tko je pogledao svaki komentar niti ne pruža statistike o tome.

Međutim, ovu značajku možemo uključiti, i tada će sustav početi pratiti kada se svaki korisnik pomakne do komentara.

Kada se to dogodi, broj pored ikone oka na svakom komentaru će se povećati. Broj se ažurira uživo i skraćuje prema lokalnim postavkama korisnika.

Ovo možemo omogućiti postavljanjem zastavice **enableViewCounts** na true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

To se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Pratimo user id* koji je pogledao komentar, tako da se ponovnim pregledom komentara broj neće povećati. Ako komentar ponovno pregledate nakon dvije godine, broj će se ponovno povećati.

- *Napomena: ili anon session id, ili IP korisnika kao haširana vrijednost.

---