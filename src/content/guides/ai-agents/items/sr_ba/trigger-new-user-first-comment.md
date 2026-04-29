Okida se kada korisnik objavi svoj prvi komentar na ovom sajtu (vaš tenant). Ovo je **jednom po korisniku** - naredni komentari istog korisnika ga ne pokreću ponovo.

### Kontekst koji agent prima

- Novi komentar.
- Opcionalni kontekst teme / istorije korisnika / stranice prema konfiguraciji.

Kada je kontekst istorije korisnika uključen, lista nedavnih komentara tog korisnika će naravno biti prazna (ili sadržavati samo ovaj), ali faktor povjerenja i starost naloga su popunjeni.

### Napomena

- "Prvi komentar na ovom sajtu" je ograničen na **tenant**, a ne na cijeli FastComments. Korisnik sa komentarima na drugim FastComments sajtovima i dalje pokreće ovaj okidač prvi put kada objavi na vašem.
- Okidač se aktivira samo za korisnike sa userId. Anonimni neprovjereni komentari bez stabilnog userId-a ga ne aktiviraju.
- Okidač se aktivira kada je komentar odobren/vidljiv (ne prilikom inicijalnog objavljivanja). Izmjene ili moderatorske akcije na prvom komentaru ga ne pokreću ponovo.

### Uobičajene upotrebe

- **Pozdrav dobrodošlice** - šablon [Šablon dobrodošlice](#template-welcome-greeter) je napravljen oko ovog okidača.
- **Uvođenje** - pošaljite [DM upozorenje](#tool-warn-user) (ovdje se koristi kao ljubazni podsjetnik, a ne stvarno upozorenje) koje usmjerava korisnika na smjernice zajednice.
- **Obavijest recenzentu** - ako želite da čovjek pogleda svaki prvi post novog komentatora, [`mark_comment_reviewed`](#tools-overview) ih može označiti za pregled.