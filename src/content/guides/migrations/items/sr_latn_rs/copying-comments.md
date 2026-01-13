Ako je potrebno da se podaci premeste, FastComments pruža alat za samouslugu za premeštanje komentara između stranica i članaka.

Evo kako izgleda forma za kopiranje komentara:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Popunjavanje polja "From"

Da bismo odlučili odakle premestiti komentare, potrebno je da znamo izvorni `URL ID`.

Ako ne prosleđujete vrednost za `urlId` u konfiguraciji widgeta za komentare, ovo će biti "čista" verzija URL-a stranice.

Možete videti koje vrednosti vaši komentari imaju za `URL ID` tako što ćete ih izvesti.

### Popunjavanje polja "To"

Da bismo odlučili gde premestiti komentare, potrebno nam je ciljni `URL ID` i `URL`.

`URL ID` predstavlja 'bucket' u koji komentar pripada. Polje `URL` se koristi da biste mogli direktno da odete do komentara iz e-poruka i alata za moderaciju.

#### WordPress

Ako koristite WordPress, na primer biste uneli ID-ove članaka u 'To'/'From' `URL ID` polja u alatu za migraciju, umesto URL-a.