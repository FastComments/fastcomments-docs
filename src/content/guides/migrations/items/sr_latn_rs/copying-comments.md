---
U slučaju da je potrebno premestiti podatke, FastComments pruža alat za samostalno premeštanje komentara između stranica i članaka.

Evo kako izgleda forma za kopiranje komentara:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Forma za kopiranje komentara sa poljem From URL ID i poljima To URL ID i URL'; title='Forma za kopiranje komentara' app-screenshot-end]

### Popunjavanje polja „From“

Da bismo odlučili odakle da premestimo komentare, potrebno je da znamo izvorni `URL ID`.

Ako ne prosleđujete vrednost za `urlId` u konfiguraciji widgeta za komentare, onda će ovo biti „čista“ verzija URL-a stranice.

Možete videti koje vrednosti vaši komentari imaju za `URL ID` tako što ćete ih izvesti.

### Popunjavanje polja „To“

Da bismo odlučili gde da premestimo komentare, potrebno je da znamo ciljni `URL ID` i `URL`.

`URL ID` će biti kantu (bucket) u koju komentar ide. Polje `URL` se koristi kako biste mogli direktno da pristupite komentaru iz e‑mailova i alata za moderaciju.

#### WordPress

Ako koristite WordPress, na primer biste uneli ID‑ove članaka u polja To/From `URL ID` u alatu za migraciju, umesto URL‑a.

---