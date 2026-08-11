[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Po defaultu, FastComments prikazuje zvono obaveštenja u gornjem desnom uglu oblasti za komentare.

Ovo zvono će postati crveno i prikazati broj obaveštenja koje korisnik ima. Neki primeri obaveštenja su:

- Korisnik vam je odgovorio.
- Korisnik je odgovorio u temi u kojoj ste komentarisali.
- Korisnik je dao glas vašem komentaru.
- Korisnik je odgovorio na stranicu na koju ste pretplaćeni.

Zvono obaveštenja takođe pruža mehanizam za pretplatu na celu stranicu.

Međutim, možemo potpuno onemogućiti zvono obaveštenja:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Ovo se može uraditi i bez koda. Na stranici za prilagođavanje widgeta, pogledajte sekciju „Disable Notification Bell“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Stranica za prilagođavanje widgeta sa poljem \"Disable Notification Bell\" označenim'; title='Onemogući zvono obaveštenja' app-screenshot-end]