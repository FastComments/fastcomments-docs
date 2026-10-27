An **AI Agent** je autonomni radnik, ograničen na vaš FastComments najam, koji prati događaje u vašoj zajednici i poduzima radnje u vaše ime.

Svaki agent ima tri stvari koje kontrolirate:

1. **Osobnost.** Besplatni tekstualni početni upit koji definira ton, ulogu i stil donošenja odluka (“Vi ste toplogobodni dočekivač zajednice”, “Vi provodite pravila zajednice, ali skloniji ste upozorenju nego zabrani”, i tako dalje).
2. **Jedan ili više okidača.** Popis događaja koji probude agenta – novi komentar, komentar koji prelazi prag glasova ili zastavica, radnja moderatora, prvi komentar korisnika na stranici i drugi. Cjeloviti popis je u [Trigger Events Overview](#triggers-overview).
3. **Popis dopuštenih alata.** Što agent smije raditi – objaviti komentar, glasati, zakačiti, zaključati, označiti kao spam, zabraniti korisnika, upozoriti putem DM-a, dodijeliti značku, poslati e‑mail, spremiti i pretražiti zajedničku memoriju. Cjeloviti popis je u [Allowed Tool Calls Overview](#tools-overview).

Kada se okidač aktivira, agent prima poruku konteksta koja opisuje što se dogodilo (komentar, stranica, opcionalni kontekst teme/korisnika/stranice) i dobiva svoj početni upit zajedno s vašim smjernicama zajednice. Zatim poziva alate za djelovanje, bilježeći opravdanje i ocjenu pouzdanosti uz svaki poziv.

### Agenti rade asinkrono

Agenti **nikada ne blokiraju radnju korisnika koja ih je pokrenula**. Čitatelj objavi komentar, komentar se spremi i prikaže u temi, odgovor se vrati, i tek *tada* agent radi na njemu – odmah ili nakon konfiguriranog odgode (pogledajte [Deferred Triggers](#trigger-deferred-delay)). Ništa što agent učini ne dodaje latenciju korisničkom iskustvu.

### Zašto ih koristiti

- **Moderirajte u velikom opsegu.** Označite očigledni spam i zabranite ponavljajuće prekršitelje bez stalnog praćenja reda.
- **Dočekajte nove komentatore.** Odgovorite na komentare prvih korisnika vašim glasom.
- **Istaknite najbolji sadržaj.** Zakačite značajne komentare na najvišoj razini kada prijeđu prag glasova.
- **Dosljedno provodite svoje smjernice.** Primijenite isti tekst politike na svaki komentar na granici.
- **Sažmite duge teme.** Objavite neutralne sažetke višestraničnih rasprava.

### Što vas drži pod kontrolom

- **Način suhog izvođenja.** Svaki novi agent isporučuje se u **Dry Run** načinu: obrađuje okidače, pokreće model i bilježi što bi *uradio*, ali ne poduzima stvarne radnje. Pogledajte [Dry-Run Mode](#dry-run-mode).
- **Odobrenja.** Bilo koji podskup radnji može biti podložan odobrenju od strane čovjeka. Pogledajte [Approval Workflow](#approval-workflow).
- **Budžeti po agentu i po računu.** Strogi dnevni i mjesečni limiti. Pogledajte [Budgets Overview](#budgets-overview).
- **Popis dopuštenih alata.** Nedopušteni alati se uklanjaju iz palete modela – agent doslovno ne može zatražiti ih. Pogledajte [Allowed Tool Calls Overview](#tools-overview).
- **Polja revizije na svakoj radnji.** Model mora sadržavati opravdanje i ocjenu pouzdanosti. Oba se pojavljuju u vremenskoj liniji izvođenja i na svakom odobrenju. Pogledajte [Run Detail View](#run-detail-view).
- **EU DSA Članak 17.** U EU regiji, potpuno automatizirane zabrane su blokirane. Pogledajte [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Nema treniranja na vašim podacima.** FastComments koristi pružatelje koji ne treniraju na vašim upitima ili komentarima.

### Gdje se uklapaju uz ljudsku moderaciju

Agent i ljudski moderatori dijele istu platformu za komentare: agenti poduzimaju radnje kroz iste kanale (odobri, spam, zabrana, značka, zakači, zaključaj, piši) i te radnje se pojavljuju u istim [Comment Logs](/guide-moderation.html#comment-logs), istoj [Moderation Page](/guide-moderation.html#moderate-comments-page) i istim tokovima obavijesti. Agenti i ljudi vide rad jedni drugih i mogu reagirati jedni na druge – radnje moderatora same po sebi su valjani okidači za agente (pogledajte [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) i slične).

Ako vam također trebaju ljudi u redu, FastComments nudi [Human Moderation Services](/guide-moderation.html#human-moderation-services): naši moderatori rade uz vaše agente, u vašoj nadzornoj ploči, prema vašim smjernicama.