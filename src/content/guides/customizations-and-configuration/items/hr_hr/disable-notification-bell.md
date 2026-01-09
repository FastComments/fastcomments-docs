---
[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments će prikazati zvono za obavijesti u gornjem desnom kutu područja za komentare.

Zvono će postati crveno i prikazat će broj obavijesti koje korisnik ima. Neki primjeri obavijesti su:

- Korisnik vam je odgovorio.
- Korisnik je odgovorio u niti u kojoj ste komentirali.
- Korisnik je glasao za vaš komentar.
- Korisnik je odgovorio na stranicu na koju ste pretplaćeni.

Zvono za obavijesti također pruža mogućnost pretplate na cijelu stranicu.

Međutim, možemo potpuno onemogućiti zvono za obavijesti:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Ovo je moguće i bez koda. Na stranici za prilagodbu widgeta pogledajte odjeljak "Onemogući zvono obavijesti".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]
---