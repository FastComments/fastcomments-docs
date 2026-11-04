**ID predloška:** `welcome_greeter`

Welcome Greeter toplo odgovara prvim komentatorima. To je predložak s najmanjim rizikom (bez destruktivnih alata) i dobar prvi agent za objavu uživo.

### Okidači

- **Novi korisnik objavljuje svoj prvi komentar na ovom sajtu** (`NEW_USER_FIRST_COMMENT`).

Ovaj se događaj aktivira točno jednom po korisniku, pa agent ne može ući u petlju. Pogledajte [Okidač: Prvi komentar novog korisnika](#trigger-new-user-first-comment).

### Dopušteni alati

- [`write_comment`](#tools-overview)

To je jedini alat - agent doslovno ne može moderirati, glasati, banirati niti slati DM.

### Preporučene dopune prije objave uživo

- **Postavite Display name** na nešto privlačno - "Community Bot", maskotu vašeg sajta ili naziv vašeg brenda. Display name je ono što čitatelji vide uz odgovor dobrodošlice.
- **Označite "Include page title, subtitle, description, and meta tags"** u [Context Options](#context-options). Odgovori dobrodošlice primjetno su bolji kada agent može referencirati o čemu stranica stvarno govori.
- **Razmotrite ograničenja po jeziku (locale)** ako djelujete na više jezika. Odgovor dobrodošlice u pogrešnom jeziku više smeta nego izostanak odgovora. Pogledajte [Opseg: URL i filteri jezika](#scope-url-locale).

### Zašto odobrenja nisu potrebna

Agent samo piše nove komentare i samo na jednokratni okidač. Najgore što se može dogoditi: nezgrapan pozdrav. Nema destruktivne radnje koju treba ograničiti. Većina operatora pokreće ovaj agent bez ikakvih odobrenja čim dry-run izgleda uredno.