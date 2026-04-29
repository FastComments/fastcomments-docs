An **AI Agent** je autonomni radnik, ograničen na vaš FastComments tenant, koji nadzire događaje u vašoj zajednici i poduzima radnje u vaše ime.

Svaki agent ima tri stvari kojima upravljate:

1. **Osobnost.** Početni prompt u slobodnom tekstu koji definira ton, ulogu i stil donošenja odluka ("Vi ste srdačni pozdravljač zajednice", "Provodite pravila zajednice, ali imate sklonost upozoravanju prije zabrane" i slično).
2. **Jedan ili više okidača.** Popis događaja koji bude agenta - novi komentar, komentar koji prelazi prag glasova ili prijava, moderatorska akcija, prvi komentar korisnika na stranici i drugi. Cjelovit popis nalazi se u [Pregled okidača](#triggers-overview).
3. **Popis dopuštenih alata.** Što je agentu dopušteno raditi - objaviti komentar, glasati, prikvačiti, zaključati, označiti kao spam, zabraniti korisnika, upozoriti putem DM-a, dodijeliti značku, poslati email, spremiti i pretražiti zajedničku memoriju. Cjelovit popis nalazi se u [Pregled dopuštenih poziva alata](#tools-overview).

Kada se okidač aktivira, agent prima poruku konteksta koja opisuje što se dogodilo (komentar, stranica, opcionalni kontekst niti/korisnika/stranice) i dobiva svoj početni prompt i smjernice vaše zajednice. Zatim poziva alate za djelovanje, bilježeći opravdanje i ocjenu povjerenja uz svaki poziv.

### Agenti rade asinhrono

Agenti **nikada ne blokiraju korisničku radnju koja ih je aktivirala**. Čitatelj pošalje komentar, komentar se sprema i prikazuje u niti, odgovor se vraća, i tek *nakon toga* agent radi na njemu — odmah ili nakon konfigurirane odgode (vidi [Odgođeni okidači](#trigger-deferred-delay)). Ništa što agent učini ne dodaje kašnjenje korisničkom iskustvu.

### Zašto ih koristiti

- **Moderirajte na velikoj skali.** Označite očiti spam i zabranite ponovljene prekršitelje bez stalnog nadzora reda.
- **Pozdravite nove komentatore.** Odgovorite prvim komentarima u vašem tonu.
- **Istaknite najbolji sadržaj.** Prikvačite sadržajne komentare na vrhu kada prijeđu prag glasova.
- **Dosljedno provodite svoje smjernice.** Primijenite isti tekst politike na svaki problematični komentar.
- **Sažmite duge rasprave.** Objavite neutralne sažetke višestranih debata.

### Što vam omogućuje kontrolu

- **Način suhog pokretanja.** Svaki novi agent dolazi u načinu **Dry Run**: obrađuje okidače, pokreće model i bilježi što bi *učinio*, ali ne poduzima stvarne akcije. Pogledajte [Način suhog pokretanja](#dry-run-mode).
- **Odobrenja.** Bilo koji podskup akcija može biti ograničen ljudskim odobrenjem. Pogledajte [Tijek odobravanja](#approval-workflow).
- **Budžeti po agentu i računu.** Strogi dnevni i mjesečni limiti. Pogledajte [Pregled budžeta](#budgets-overview).
- **Popis dopuštenih alata.** Alati koji nisu dopušteni uklanjaju se iz modelove palete — agent ih doslovno ne može zatražiti. Pogledajte [Pregled dopuštenih poziva alata](#tools-overview).
- **Polja za reviziju na svakoj akciji.** Model mora uključiti opravdanje i ocjenu povjerenja. Oba se pojavljuju u vremenskoj liniji izvođenja i pri svakom odobravanju. Pogledajte [Pregled detalja izvođenja](#run-detail-view).
- **EU DSA Članak 17.** U regiji EU-a, potpuno automatizirane zabrane su blokirane. Pogledajte [Usklađenost s EU DSA Članak 17](#eu-dsa-compliance).
- **Bez treniranja na vašim podacima.** FastComments koristi pružatelje koji ne treniraju na vašim promptovima ili komentarima.

### Gdje se uklapaju uz ljudsko moderiranje

Agenti i ljudski moderatori dijele istu platformu za komentare: agenti poduzimaju radnje kroz iste kanale (approve, spam, ban, badge, pin, lock, write) i te se radnje pojavljuju u istim [Zapisnicima komentara](/guide-moderation.html#comment-logs), na istoj [Stranici za moderiranje](/guide-moderation.html#moderate-comments-page) i u istim tokovima obavijesti. Agenti i ljudi vide rad jedni drugih i mogu reagirati jedni na druge — moderatorske akcije same po sebi su valjani okidači za agente (vidi [Okidač: Moderator je pregledao komentar](#trigger-moderator-reviewed) i povezane).