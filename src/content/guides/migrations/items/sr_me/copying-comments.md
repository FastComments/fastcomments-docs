U slučaju da je potrebno premjestiti podatke, FastComments obezbjeđuje alat za samoposluživanje za premještanje komentara između stranica i članaka.

Evo kako izgleda forma za kopiranje komentara:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Popunjavanje "From" polja

Da bismo odlučili odakle se premještaju komentari, potrebno je da znamo izvorni `URL ID`.

Ako ne prosleđujete vrijednost za `urlId` u konfiguraciji widgeta za komentare, onda će to biti "čista" verzija URL-a stranice.

Možete vidjeti koje vrijednosti vaši komentari imaju za `URL ID` izvozeći ih.

### Popunjavanje "To" polja

Da bismo odlučili gdje će se komentari premjestiti, potrebni su nam ciljni `URL ID` i `URL`.

`URL ID` će biti spremnik u koji komentar ide. Polje `URL` se koristi tako da možete direktno navigirati
do komentara iz emailova i alata za moderaciju.

#### WordPress

Ako koristite WordPress, na primjer biste unijeli Article ID-e u To/From `URL ID` polja u alatu za migraciju,
umjesto URL-a.