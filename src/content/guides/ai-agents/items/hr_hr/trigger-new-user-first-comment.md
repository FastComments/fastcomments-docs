Okida se kada korisnik objavi svoj prvi komentar na ovoj stranici (vaš tenant). Ovo je **jednom po korisniku** - naknadni komentari istog korisnika ga ne ponovno pokreću.

### Kontekst koji agent prima

- Novi komentar.
- Opcionalni kontekst niti / povijest korisnika / kontekst stranice kako je konfigurirano.

Kada je uključen kontekst povijesti korisnika, popis nedavnih komentara korisnika bit će naravno prazan (ili će sadržavati samo ovaj), ali su faktor povjerenja i starost računa popunjeni.

### Napomena

- "Prvi komentar na ovoj stranici" je ograničen na **tenant**, a ne na cijeli FastComments. Korisnik koji ima komentare na drugim FastComments stranicama i dalje će pokrenuti ovaj okidač prvi put kad objavi na vašoj.
- Okidač se aktivira samo za korisnike koji imaju userId. Anonimni neprovjereni komentari bez stabilnog userId-a ga ne aktiviraju.
- Okidač se aktivira kada je komentar odobren/vidljiv (ne u trenutku prvotnog objavljivanja). Uređivanja ili moderatorijske radnje na prvim komentarima ga ne ponovno pokreću.

### Uobičajene upotrebe

- **Pozdrav dobrodošlice** - [Predložak Welcome Greeter](#template-welcome-greeter) je izgrađen oko ovog okidača.
- **Uvođenje** - pošaljite [DM upozorenje](#tool-warn-user) (ovdje se koristi kao prijateljski podsjetnik, a ne kao upozorenje) koje korisnika usmjerava na smjernice zajednice.
- **Obavijest recenzenta** - ako želite da čovjek pogleda prvu objavu svakog novog komentatora, [`mark_comment_reviewed`](#tools-overview) ih može označiti za pregled.

---