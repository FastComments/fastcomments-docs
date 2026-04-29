**ID predloška:** `welcome_greeter`

Welcome Greeter srdačno odgovara korisnicima koji komentiraju prvi put. To je najmanje rizičan predložak (bez destruktivnih alata) i dobar prvi agent za puštanje uživo.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt predloška Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Okidači

- **Novi korisnik objavi svoj prvi komentar na ovom web-mjestu** (`NEW_USER_FIRST_COMMENT`).

Ovaj događaj se pokreće točno jednom po korisniku, tako da agent ne može ući u petlju. Pogledajte [Okidač: Novi korisnik — prvi komentar](#trigger-new-user-first-comment).

### Dozvoljeni alati

- [`write_comment`](#tools-overview)

To je jedini alat — agent doslovno ne može moderirati, glasati, zabranjivati ili slati privatne poruke (DM).

### Preporučene dopune prije puštanja uživo

- **Postavite prikazano ime** na nešto privlačno - "Community Bot", maskota vaše stranice, ili naziv vašeg brenda. Prikazano ime je ono što čitatelji vide uz odgovor dobrodošlice.
- **Označite "Uključi naslov stranice, podnaslov, opis i meta oznake"** u [Opcije konteksta](#context-options). Odgovori pozdravljača postaju značajno bolji kad može referencirati o čemu je stranica.
- **Razmotrite ograničenja lokaliteta** ako poslujete na više jezika. Poruka dobrodošlice na pogrešnom jeziku je uznemirujuća više nego propušten odgovor. Pogledajte [Opseg: Filteri URL-a i lokaliteta](#scope-url-locale).

### Zašto odobrenja nisu potrebna

Agent samo piše nove komentare i samo na jedinstveni okidač. U najgorem slučaju: nezgodan pozdrav. Nema destruktivne akcije koju treba kontrolirati. Većina operatera pokreće ovaj bez ikakvih odobrenja čim probni rad izgleda uredno.

---