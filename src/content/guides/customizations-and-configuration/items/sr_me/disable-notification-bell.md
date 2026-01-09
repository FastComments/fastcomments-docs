[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments će prikazati zvono za obavještenja u gornjem desnom uglu područja za komentare.

To zvono postaće crveno i prikazaće broj obavještenja koje korisnik ima. Neki primjeri obavještenja su:

- Korisnik vam je odgovorio.
- Korisnik je odgovorio u temi u kojoj ste komentarisali.
- Korisnik je glasao za vaš komentar.
- Korisnik je odgovorio na stranicu na koju ste pretplaćeni.

Zvono za obavještenja takođe pruža mehanizam za pretplatu na čitavu stranicu.

Međutim, možemo potpuno onemogućiti zvono za obavještenja:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Ovo se može uraditi i bez koda. Na stranici za prilagođavanje widgeta pogledajte odjeljak "Onemogući zvono za obavještenja".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]