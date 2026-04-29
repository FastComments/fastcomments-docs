**Template ID:** `welcome_greeter`

Pozdravnik dobrodošlice srdačno odgovara korisnicima koji prvi put komentiraju. To je predložak s najmanjim rizikom (bez destruktivnih alata) i dobar je prvi agent za puštanje uživo.

### Triggers

- **Novi korisnik objavi svoj prvi komentar na ovoj stranici** (`NEW_USER_FIRST_COMMENT`).

Ovaj događaj pokreće se točno jednom po korisniku, tako da agent ne može ući u petlju. Vidi [Okidač: Novi korisnik - prvi komentar](#trigger-new-user-first-comment).

### Allowed tools

- [`write_comment`](#tools-overview)

To je jedini alat - agent doslovno ne može moderirati, glasovati, zabranjivati ili slati privatne poruke (DM).

### Recommended additions before going live

- **Postavite prikazano ime** na nešto primamljivo - "Community Bot", maskotu vaše stranice ili naziv vašeg brenda. Prikazano ime je ono što čitatelji vide uz odgovor dobrodošlice.
- **Označite "Uključi naslov stranice, podnaslov, opis i meta oznake"** u [Opcije konteksta](#context-options). Odgovori pozdravitelja znatno se poboljšaju kada može referencirati o čemu je stranica.
- **Razmotrite ograničenja jezika (lokale)** ako poslujete na više jezika. Pozdravna poruka na pogrešnom jeziku je neugodnija od propuštene poruke. Vidi [Opseg: URL i filtri jezika](#scope-url-locale).

### Why no approvals are needed

Agent piše samo nove komentare i samo pri jednokratnom okidaču. U najgorem slučaju: neugodan pozdrav. Nema destruktivne radnje koju treba kontrolirati. Većina operatora pokreće ovaj predložak bez ikakvih odobrenja nakon što probni rad izgleda uredno.

---