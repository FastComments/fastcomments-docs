Pokreće agenta svaki put kada se na stranici koja je obuhvaćena agentovim [opseg](#scope-url-locale) postavi novi komentar.

### Kontekst koji agent prima

- Novi komentar u cjelosti - tekst, autor, glasovi, parent ID, page URL ID.
- Opcionalno: roditeljski komentar i prethodni odgovori u istoj niti, ako je [kontekst niti](#context-options) uključen.
- Opcionalno: faktor povjerenja komentatora, starost naloga, historija zabrana i nedavni komentari, ako je [kontekst korisničke historije](#context-options) uključen.
- Opcionalno: metapodaci stranice, ako je [kontekst stranice](#context-options) uključen.

### Napomene

- Okidač se pokreće **nakon** što je komentar sačuvan. Agent može ga direktno referencirati u pozivima alata.
- Ne pokreće se za komentare koje je napisao drugi agent u istom tenantu.
- Pokreće se za i verifikovane i neverifikovane komentare. Ako vaš tenant zahtijeva odobrenje moderatora prije nego što komentar postane vidljiv (pogledajte [Kako odobrenja funkcionišu](/guide-moderation.html#moderation-approvals) u vodiču za moderaciju), okidač se pokreće kada je komentar kreiran, a ne kada je naknadno odobren. Moderator bot se može uputiti da odobri komentare umjesto vas nakon pregleda.

### Uobičajene upotrebe

- **Moderacija** - provjerite komentar u odnosu na smjernice zajednice, označite kao spam ili upozorite korisnike koji prvi put komentiraju.
- **Pozdrav dobrodošlice** - iako je [Trigger: New User First Comment](#trigger-new-user-first-comment) obično prikladniji za pozdrave jer se pokreće jednom po korisniku.
- **Sumiranje niti** - obično se koristi u paru sa [odgoda okidača](#trigger-deferred-delay) tako da se nit stabilizuje prije nego agent pokrene.