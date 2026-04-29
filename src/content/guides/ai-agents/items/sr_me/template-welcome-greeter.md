**ID шаблона:** `welcome_greeter`

Pozdravni agent srdačno odgovara korisnicima који prvi put komentarišu. To je predložak с најмањим ризиком (без destruktivnih alata) i dobar prvi agent za puštanje uživo.

### Okidači

- **Novi korisnik objavljuje svoj prvi komentar na ovom sajtu** (`NEW_USER_FIRST_COMMENT`).

Ovaj događaj se pokreće tačno jednom po korisniku, tako da agent ne može da petlja. Pogledajte [Okidač: Novi korisnik - prvi komentar](#trigger-new-user-first-comment).

### Dozvoljeni alati

- [`write_comment`](#tools-overview)

To je jedini alat — agent bukvalno ne može da moderira, glasa, blokira ili šalje direktne poruke (DM).

### Preporučene dopune pre puštanja uživo

- **Postavite prikazano ime** na nešto pozivajuće - "Community Bot", maskota vašeg sajta, ili ime vašeg brenda. Prikazano ime je ono što čitaoci vide uz odgovor dobrodošlice.
- **Označite "Uključi naslov stranice, podnaslov, opis i meta tagove"** u [Opcije konteksta](#context-options). Odgovori pozdravnog agenta postaju primetno bolji kada može da se pozove na to o čemu stranica zapravo govori.
- **Razmotrite ograničenja lokaliteta** ako poslujete na više jezika. Odgovor dobrodošlice na pogrešnom jeziku je više uznemirujući nego propušten odgovor. Pogledajte [Opseg: URL i filteri lokaliteta](#scope-url-locale).

### Zašto odobrenja nisu potrebna

Agent piše samo nove komentare i samo na jednokratni okidač. Najgori ishod: neprijatan pozdrav. Ne postoji destruktivna akcija koju treba kontrolisati. Većina operatera koristi ovaj bez ikakvih odobrenja čim probni rad izgleda uredno.