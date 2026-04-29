**ID predloška:** `welcome_greeter`

Welcome Greeter odgovara srdačno prvim komentatorima. To je najmanje rizičan predložak (bez destruktivnih alata) i dobar prvi agent za puštanje u rad.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt predloška Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Okidači

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

Ovaj događaj se aktivira tačno jednom po korisniku, tako da agent ne može ući u petlju. Pogledajte [Okidač: Prvi komentar novog korisnika](#trigger-new-user-first-comment).

### Dozvoljeni alati

- [`write_comment`](#tools-overview)

To je jedini alat — agent bukvalno ne može moderisati, glasati, banovati ili slati direktne poruke.

### Preporučena podešavanja prije pokretanja uživo

- **Postavite prikazano ime** na nešto pozivajuće - "Bot zajednice", maskota vašeg sajta, ili naziv vašeg brenda. Prikazano ime je ono što čitaoci vide uz odgovor dobrodošlice.
- **Označite "Uključi naslov stranice, podnaslov, opis i meta tagove"** u [Opcije konteksta](#context-options). Odgovori pozdravljača postaju primjetno bolji kada može referisati na šta stranica zapravo govori.
- **Razmotrite ograničenja lokaliteta** ako radite na više jezika. Odgovor dobrodošlice na pogrešnom jeziku je neprijatniji od propuštenog odgovora. Pogledajte [Opseg: Filteri za URL i lokalitet](#scope-url-locale).

### Zašto odobrenja nisu potrebna

Agent samo piše nove komentare i to samo na jednokratni okidač. U najgorem slučaju: nezgrapan pozdrav. Nema destruktivnih radnji koje treba ograničiti. Većina operatera pokreće ovaj bez ikakvih odobrenja nakon što probni rad izgleda uredno.