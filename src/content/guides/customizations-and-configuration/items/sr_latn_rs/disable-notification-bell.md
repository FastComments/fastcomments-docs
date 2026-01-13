[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments će prikazati ikonicu zvona za obaveštenja u gornjem desnom uglu oblasti za komentare.

To zvono će postati crveno i prikazati broj obaveštenja koje korisnik ima. Neki primeri obaveštenja su:

- Korisnik vam je odgovorio.
- Korisnik je odgovorio u niti na kojoj ste komentarisali.
- Korisnik je glasao za vaš komentar.
- Korisnik je odgovorio na stranicu na kojoj ste pretplaćeni.

Ikonica za obaveštenja takođe omogućava mehanizam za pretplatu na celu stranicu.

Međutim, možemo u potpunosti onemogućiti ikonicu zvona za obaveštenja:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak 'Onemogući zvono za obaveštenja'.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]