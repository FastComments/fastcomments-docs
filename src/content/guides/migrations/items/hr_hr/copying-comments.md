U slučaju da je potrebno premjestiti podatke, FastComments nudi alat za samoposluživanje za premještanje komentara
između stranica i članaka.

Evo kako izgleda obrazac stranice za kopiranje komentara:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Obrazac za kopiranje komentara s poljem From URL ID te poljima To URL ID i URL'; title='Obrazac za kopiranje komentara' app-screenshot-end]

### Ispunjavanje polja "From"

Da bismo odlučili odakle premjestiti komentare, jednostavno trebamo znati izvorni `URL ID`.

Ako ne prosljeđujete vrijednost za `urlId` u konfiguraciji widgeta za komentare, tada će to biti "očišćena" verzija URL-a stranice.

Možete vidjeti koje vrijednosti vaši komentari imaju za `URL ID` izvozom istih.

### Ispunjavanje polja "To"

Da bismo odlučili kamo premjestiti komentare, trebamo znati odredišni `URL ID` i `URL`.

`URL ID` bit će grupa u koju komentar ide. Polje `URL` se koristi kako biste mogli izravno
navigirati do komentara iz e-poruka i alata za moderiranje.

#### WordPress

Ako koristite WordPress, na primjer biste unijeli ID-jeve članaka u polja To/From `URL ID` u alatu za migraciju,
umjesto URL-a.