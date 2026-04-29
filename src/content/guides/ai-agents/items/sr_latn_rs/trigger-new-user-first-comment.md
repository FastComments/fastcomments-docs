Pokreće se kada korisnik objavi svoj prvi komentar na ovom sajtu (vaš tenant). Ovo je **jednom po korisniku** - naredni komentari istog korisnika ga ne ponovo pokreću.

### Kontekst koji agent prima

- Novi komentar.
- Opcioni kontekst niti / istorije korisnika / stranice prema konfiguraciji.

Kada je uključen kontekst istorije korisnika, lista nedavnih komentara korisnika će naravno biti prazna (ili sadržati samo ovaj), ali faktor poverenja i starost naloga su popunjeni.

### Bitno

- "First comment on this site" je ograničeno na **tenant**, ne na nivou cele platforme FastComments. Korisnik koji ima komentare na drugim FastComments sajtovima i dalje pokreće ovaj okidač prvi put kada objavi na vašem.
- Okidač se pokreće samo za korisnike koji imaju userId. Anonimni neverifikovani komentari bez stabilnog userId ne pokreću ga.
- Okidač se aktivira kada je komentar odobren/vidljiv (ne u trenutku inicijalne objave). Izmene ili radnje moderatora na prvim komentarima ga ne ponovo aktiviraju.

### Uobičajene upotrebe

- **Pozdrav dobrodošlice** - [Welcome Greeter šablon](#template-welcome-greeter) je izgrađen oko ovog okidača.
- **Onboarding** - pošaljite [DM upozorenje](#tool-warn-user) (ovde se koristi kao ljubazno obaveštenje, a ne kao upozorenje) koje korisnika upućuje na smernice zajednice.
- **Obaveštenje recenzenta** - ako želite da čovek pregleda prvi post svakog novog komentatora, [`mark_comment_reviewed`](#tools-overview) može ih označiti za pregled.

---