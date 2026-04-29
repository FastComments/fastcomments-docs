---
**ID šablona:** `welcome_greeter`

Welcome Greeter srdačno odgovara korisnicima koji prvi put komentarišu. To je najmanje rizičan šablon (bez destruktivnih alata) i dobar prvi agent za puštanje uživo.

### Okidači

- **Novi korisnik ostavi svoj prvi komentar na ovom sajtu** (`NEW_USER_FIRST_COMMENT`).

Ovaj događaj se aktivira tačno jednom po korisniku, tako da agent ne može da uđe u petlju. Pogledajte [Okidač: Novi korisnik - prvi komentar](#trigger-new-user-first-comment).

### Dozvoljeni alati

- [`write_comment`](#tools-overview)

To je jedini alat — agent bukvalno ne može da moderira, glasa, blokira ili šalje DM.

### Preporučene dopune pre puštanja uživo

- **Podesite prikazano ime** na nešto pozivajuće — "Community Bot", maskota vašeg sajta, ili ime vašeg brenda. Prikazano ime je ono što čitaoci vide uz odgovor dobrodošlice.
- **Označite "Uključi naslov stranice, podnaslov, opis i meta tagove"** u [Context Options](#context-options). Odgovori pozdravljača postaju primetno bolji kada može da se pozove na to o čemu stranica zapravo govori.
- **Razmotrite ograničenja lokalizacije** ako poslujete na više jezika. Odgovor dobrodošlice na pogrešnom jeziku je neprijatniji od propuštenog odgovora. Pogledajte [Scope: URL and Locale Filters](#scope-url-locale).

### Zašto odobrenja nisu potrebna

Agent samo piše nove komentare i to samo na jednokratni okidač. Najgori slučaj: neprijatan pozdrav. Ne postoji destruktivna akcija koju treba ograničiti. Većina operatera pokreće ovaj šablon bez ikakvih odobrenja nakon što rezultati u probnom režimu deluju čisto.

---