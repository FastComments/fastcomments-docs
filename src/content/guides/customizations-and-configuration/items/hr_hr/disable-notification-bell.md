[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Po zadanom, FastComments će prikazati zvono obavijesti u gornjem desnom kutu područja za komentare.

Ovo zvono će postati crveno i prikazati broj obavijesti koje korisnik ima. Neki primjeri obavijesti su:

- Korisnik vam je odgovorio.
- Korisnik je odgovorio u temi u kojoj ste komentirali.
- Korisnik je glasao za vaš komentar.
- Korisnik je odgovorio na stranicu na koju ste pretplaćeni.

Zvono obavijesti također pruža mehanizam za pretplatu na cijelu stranicu.

Međutim, možemo potpuno onemogućiti zvono obavijesti:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Onemogući zvono obavijesti'; code-example-end]

Ovo se također može učiniti bez koda. Na stranici za prilagodbu widgeta, pogledajte odjeljak „Onemogući zvono obavijesti“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Stranica za prilagodbu widgeta s označenim potvrdnim okvirom Onemogući zvono obavijesti'; title='Onemogući zvono obavijesti' app-screenshot-end]