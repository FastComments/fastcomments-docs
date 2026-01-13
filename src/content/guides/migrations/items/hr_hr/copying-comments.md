U slučaju da je potrebno premjestiti podatke, FastComments pruža alat za samoposlugu za premještanje komentara
između stranica i članaka.

Evo kako izgleda obrazac za kopiranje komentara:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Popunjavanje polja "Iz"

Da bismo odlučili odakle premjestiti komentare, trebamo znati izvorni `URL ID`.

Ako ne prosljeđujete vrijednost za `urlId` u konfiguraciji widgeta za komentare, tada će ovo biti "čista" verzija URL-a stranice.

Možete vidjeti koje vrijednosti vaši komentari imaju za `URL ID` izvozenjem.

### Popunjavanje polja "Za"

Da bismo odlučili kamo premjestiti komentare, trebamo znati ciljni `URL ID` i `URL`.

`URL ID` će biti spremnik u koji komentar ide. Polje `URL` se koristi tako da možete izravno
navigirati do komentara iz e-poruka i alata za moderaciju.

#### WordPress

Ako koristite WordPress, primjerice biste unijeli ID-eve članaka u polja "Za"/"Iz" `URL ID` u alatu za migraciju,
umjesto URL-a.