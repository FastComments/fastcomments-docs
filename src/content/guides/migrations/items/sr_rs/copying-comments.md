U slučaju da podaci moraju da se premeste, FastComments obezbeđuje alat za samo-uslužno premeštanje komentara
između stranica i članaka.

Ovako izgleda obrazac za kopiranje komentara:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Popunjavanje polja "From"

Da bismo odlučili odakle da premestimo komentare, potrebno je samo da znamo izvorni `URL ID`.

Ako ne prosleđujete vrednost za `urlId` u konfiguraciji komentarskog vidžeta, onda će ovo biti "čista" verzija URL-a stranice.

Možete videti koje vrednosti za `URL ID` imaju vaši komentari tako što ćete ih izvesti.

### Popunjavanje polja "To"

Da bismo odlučili gde da premestimo komentare, potrebno je da znamo ciljni `URL ID` i `URL`.

`URL ID` će biti korpa u kojoj će komentar biti smešten. Polje `URL` se koristi tako da možete direktno da odete do komentara iz e-poruka i alata za moderaciju.

#### WordPress

Ako koristite WordPress, na primer biste uneli ID-ove članaka u To/From `URL ID` polja u alatu za migraciju, umesto URL-a.