**Template ID:** `welcome_greeter`

Agent Dobrodošlice srdačno odgovara korisnicima koji prvi put komentarišu. To je predložak sa najmanjim rizikom (bez destruktivnih alata) i dobar prvi agent za puštanje u produkciju.

### Okidači

- **Novi korisnik objavi svoj prvi komentar na ovom sajtu** (`NEW_USER_FIRST_COMMENT`).

Ovaj događaj se aktivira tačno jednom po korisniku, tako da agent ne može da se ponavlja. Vidi [Okidač: New User First Comment](#trigger-new-user-first-comment).

### Dozvoljeni alati

- [`write_comment`](#tools-overview)

To je jedini alat — agent zapravo ne može da moderira, glasa, banuje ili šalje privatne poruke.

### Preporučene dopune pre puštanja uživo

- **Podesite prikazano ime** na nešto pristupačno - "Community Bot", maskota sajta, ili ime vašeg brenda. Prikazano ime je ono što čitaoci vide prikačeno uz pozdravni odgovor.
- **Označite "Uključi naslov stranice, podnaslov, opis i meta tagove"** u [Opcije konteksta](#context-options). Odgovori agenta dobrodošlice postaju primetno bolji kada može da se pozove na to o čemu stranica zapravo govori.
- **Razmotrite ograničenja za lokalizaciju** ako poslujete na više jezika. Pozdrav na pogrešnom jeziku više uznemirava nego propušteni odgovor. Vidi [Opseg: URL and Locale Filters](#scope-url-locale).

### Zašto odobrenja nisu potrebna

Agent samo piše nove komentare i samo na jednokratni okidač. U najgorem slučaju: nezgrapan pozdrav. Nema destruktivne akcije koja zahteva kontrolu. Većina operatera pokreće ovog agenta bez ikakvih odobrenja kada probni rad (dry-run) izgleda u redu.