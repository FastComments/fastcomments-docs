---
**AI Agent** je autonomni radnik, ograničen na vaš FastComments tenant, koji prati događaje u vašoj zajednici i preduzima akcije u vaše ime.

Svaki agent ima tri stvari koje vi kontrolišete:

1. **Ličnost.** Slobodni tekst početnog upita koji definiše ton, ulogu i stil donošenja odluka („Vi ste toplog duha dočekivač zajednice“, „Vi sprovodite pravila zajednice, ali više naginjete ka upozorenju nego zabrani“, i slično).
2. **Jedan ili više okidača.** Lista događaja koji bude agenta – novi komentar, komentar koji pređe prag glasova ili zastavica, akcija moderatora, prvi komentar korisnika na sajtu i drugi. Potpuna lista je u [Trigger Events Overview](#triggers-overview).
3. **Lista dozvoljenih alata.** Šta agent sme da radi – objaviti komentar, glasati, zakačiti, zaključati, označiti kao spam, zabraniti korisnika, upozoriti putem DM‑a, dodeliti značku, poslati e‑mail, sačuvati i pretražiti deljenu memoriju. Potpuna lista je u [Allowed Tool Calls Overview](#tools-overview).

Kada se okidač aktivira, agent prima poruku konteksta koja opisuje šta se dogodilo (komentar, stranica, opcioni kontekst teme/korisnika/stranice) i dobija svoj početni upit zajedno sa vašim smernicama zajednice. Zatim poziva alate da deluje, beležeći obrazloženje i skor pouzdanosti uz svaki poziv.

### Agenti rade asinhrono

Agenti **nikada ne blokiraju radnju korisnika koja ih je pokrenula**. Čitač objavi komentar, komentar se sačuva i prikaže u temi, odgovor se vrati, i tek *tada* agent izvrši radnju – odmah ili nakon podešenog kašnjenja (pogledajte [Deferred Triggers](#trigger-deferred-delay)). Ništa što agent uradi ne dodaje latenciju korisničkom iskustvu.

### Zašto ih koristiti

- **Moderacija u velikom obimu.** Označite očigledni spam i zabranite ponovljenim prekršiocima bez stalnog praćenja reda.
- **Dočekajte nove komentatore.** Odgovorite prvim komentatorima u vašem glasu.
- **Istaknite najbolji sadržaj.** Zakačite značajne komentare na najvišem nivou kada pređu prag glasova.
- **Dosledno sprovodite smernice.** Primijenite isti tekst politike na svaki komentar na granici.
- **Sažmite duge teme.** Objavite neutralne sažetke višestraničnih rasprava.

### Šta vas drži pod kontrolom

- **Modus suve probe.** Svaki novi agent se isporučuje u **Dry Run** režimu: obrađuje okidače, pokreće model i beleži šta bi *uradio*, ali ne preduzima stvarne akcije. Pogledajte [Dry-Run Mode](#dry-run-mode).
- **Odobrenja.** Bilo koji podskup akcija može biti uslovljen odobrenjem od strane čoveka. Pogledajte [Approval Workflow](#approval-workflow).
- **Budžeti po agentu i po nalogu.** Stroga dnevna i mesečna ograničenja. Pogledajte [Budgets Overview](#budgets-overview).
- **Lista dozvoljenih alata.** Nedozvoljeni alati se uklanjaju iz palete modela – agent doslovno ne može da ih zahteva. Pogledajte [Allowed Tool Calls Overview](#tools-overview).
- **Polja revizije na svakoj akciji.** Model mora da sadrži obrazloženje i skor pouzdanosti. Obe se pojavljuju u vremenskoj liniji izvršavanja i na svakom odobrenju. Pogledajte [Run Detail View](#run-detail-view).
- **EU DSA član 17.** U EU regionu, potpuno automatizovane zabrane su blokirane. Pogledajte [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Bez treniranja na vašim podacima.** FastComments koristi provajdere koji ne treniraju na vašim upitima ili komentarima.

### Gde se uklapaju uz ljudsku moderaciju

Agenti i ljudski moderatori dele istu platformu za komentare: agenti preduzimaju akcije kroz iste kanale (odobri, spam, zabrana, značka, zakačaj, zaključaj, piši) i te akcije se pojavljuju u istim [Comment Logs](/guide-moderation.html#comment-logs), istoj [Moderation Page](/guide-moderation.html#moderate-comments-page) i istim tokovima obaveštenja. Agenti i ljudi vide rad jedni drugih i mogu reagovati jedni na druge – akcije moderatora su same po sebi validni okidači za agente (pogledajte [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) i slične).

Ako vam takođe trebaju ljudi u redu, FastComments nudi [Human Moderation Services](/guide-moderation.html#human-moderation-services): naši moderatori rade uz vaše agente, u vašem kontrolnom panelu, pod vašim smernicama.
---