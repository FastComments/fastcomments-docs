---
Pokreće agenta svaki put kada se na stranici obuhvaćenoj agentovim [opsegom](#scope-url-locale) objavi novi komentar.

### Kontekst koji agent prima

- Novi komentar u cijelosti - tekst, autor, glasovi, ID roditelja, ID URL stranice.
- Opcionalno: roditeljski komentar i prethodni odgovori u istoj niti, ako je uključen [kontekst niti](#context-options).
- Opcionalno: stupanj povjerenja komentatora, dob računa, povijest zabrana i nedavni komentari, ako je uključen [kontekst povijesti korisnika](#context-options).
- Opcionalno: metapodaci stranice, ako je uključen [kontekst stranice](#context-options).

### Napomena

- Okidač se aktivira **nakon** što je komentar pohranjen. Agent mu može izravno pristupiti u pozivima alata.
- Ne aktivira se za komentare koje je napisao drugi agent u istom tenantu.
- Aktivira se i za verificirane i za_neverificirane_ komentare. Ako vaš tenant zahtijeva odobrenje moderatora prije nego što komentar postane vidljiv (pogledajte [Kako funkcionira odobravanje](/guide-moderation.html#moderation-approvals) u vodiču za moderaciju), okidač se aktivira kad je komentar kreiran, a ne kad je naknadno odobren. Moderatorski bot može se uputiti da nakon pregleda odobri komentare umjesto vas.

### Uobičajene uporabe

- **Moderacija** - provjerite komentar u odnosu na smjernice zajednice, označite kao spam ili upozorite nove korisnike.
- **Pozdrav dobrodošlice** - iako je [Okidač: Prvi komentar novog korisnika](#trigger-new-user-first-comment) obično prikladniji za pozdrave jer se aktivira jednom po korisniku.
- **Sažimanje niti** - obično u paru s [odgodom okidača](#trigger-deferred-delay) kako bi se nit smirila prije nego agent radi.

---