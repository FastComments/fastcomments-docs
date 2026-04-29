Pokreće agenta svaki put kada se na stranici obuhvaćenoj [opsegom](#scope-url-locale) agenta objavi novi komentar.

### Kontekst koji agent prima

- Novi komentar u celosti - tekst, autor, glasovi, ID roditelja, ID URL stranice.
- Opcionalno: roditeljski komentar i prethodni odgovori u istoj niti, ako je [thread context](#context-options) uključen.
- Opcionalno: faktor poverenja komentatora, starost naloga, istorija banovanja i nedavni komentari, ako je [user history context](#context-options) uključen.
- Opcionalno: metapodaci stranice, ako je [page context](#context-options) uključen.

### Napomene

- Okidač se aktivira **posle** nego što je komentar sačuvan. Agent može da se pozove na njega direktno u pozivima alata.
- Ne aktivira se za komentare koje je napisao drugi agent u istom tenantu.
- Aktivira se i za verifikovane i za neverifikovane komentare. Ako vaš tenant zahteva odobrenje moderatora pre nego što komentar bude vidljiv (vidi [Kako funkcionišu odobrenja](/guide-moderation.html#moderation-approvals) u vodiču za moderaciju), okidač se aktivira kada je komentar kreiran, a ne kada je kasnije odobren. Moderator bot može biti naložen da odobri komentare za vas nakon pregleda.

### Uobičajene upotrebe

- **Moderacija** - proverite komentar u odnosu na smernice zajednice, označite spam ili upozorite korisnike koji prvi put pišu.
- **Pozdrav dobrodošlice** - iako je [Okidač: Prvi komentar novog korisnika](#trigger-new-user-first-comment) obično pogodniji za pozdrave jer se pokreće jednom po korisniku.
- **Sumiranje niti** - obično upareno sa [odlaganje okidača](#trigger-deferred-delay) tako da se nit smiri pre nego što agent radi.