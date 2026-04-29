An **AI agent** je autonomni radnik, ograničen na vaš FastComments tenant, koji prati događaje u vašoj zajednici i djeluje u vaše ime.

Svaki agent ima tri stvari koje vi kontrolišete:

1. **Osobnost.** Početni prompt slobodnog teksta koji definiše ton, ulogu i stil donošenja odluka ("Vi ste srdačni pozdravljač zajednice", "Provodite pravila zajednice ali težite upozorenju umjesto zabrane" i tako dalje).
2. **Jedan ili više okidača.** Lista događaja koji bude agenta — novi komentar, komentar koji prelazi prag glasova ili prijava, moderatorova radnja, prvi komentar korisnika na stranici i drugi. Cijeli popis je u [Trigger Events Overview](#triggers-overview).
3. **Lista dozvoljenih alata.** Šta agent smije raditi — objaviti komentar, glasati, prikvačiti, zaključati, označiti kao spam, zabraniti korisnika, upozoriti putem DM-a, dodijeliti značku, poslati email, sačuvati i pretražiti zajedničku memoriju. Cijeli popis je u [Allowed Tool Calls Overview](#tools-overview).

Kada se okidač aktivira, agent prima kontekstualnu poruku koja opisuje šta se dogodilo (komentar, stranica, opcionalni kontekst teme/korisnika/stranice) i dobija svoj početni prompt i pravila vaše zajednice. Zatim poziva alate da djeluje, pritom bilježeći opravdanje i ocjenu povjerenja za svaki poziv.

### Agenti rade asinhrono

Agenti **nikada ne blokiraju korisničku radnju koja ih je pokrenula**. Čitalac objavi komentar, komentar se snimi i prikaže u temi, odgovor se vrati, i tek *onda* agent radi na njemu — ili odmah ili nakon konfiguriranog kašnjenja (pogledajte [Deferred Triggers](#trigger-deferred-delay)). Ništa što agent radi ne dodaje latenciju korisničkom iskustvu.

### Zašto ih koristiti

- **Moderirajte u velikom obimu.** Označite očigledan spam i zabranite ponavljače bez stalnog nadgledanja reda.
- **Pozdravite nove komentatore.** Odgovorite prvim komentatorima vašim tonom.
- **Istaknite najbolji sadržaj.** Prikvačite značajne komentare prve razine kad pređu prag glasova.
- **Dosljedno sprovodite smjernice.** Primjenjujte isti tekst pravila na svaki granični komentar.
- **Sažmite duge teme.** Objavite neutralne sažetke višestraničnih debata.

### Šta vam daje kontrolu

- **Režim probnog rada.** Svaki novi agent dolazi u **Režimu probnog rada**: procesuira okidače, pokreće model i evidentira šta bi uradio, ali ne preduzima stvarne akcije. Pogledajte [Dry-Run Mode](#dry-run-mode).
- **Odobrenja.** Bilo koji podskup radnji može biti uslovljen ljudskim odobrenjem. Pogledajte [Approval Workflow](#approval-workflow).
- **Budžeti po agentu i po računu.** Strogi dnevni i mjesečni limiti. Pogledajte [Budgets Overview](#budgets-overview).
- **Lista dozvoljenih alata.** Nedozvoljeni alati se uklanjaju iz palete modela — agent ih bukvalno ne može zatražiti. Pogledajte [Allowed Tool Calls Overview](#tools-overview).
- **Revizijska polja na svakoj radnji.** Model mora uključiti opravdanje i ocjenu povjerenja. Oba se pojavljuju u vremenskoj liniji pokretanja i pri svakom odobrenju. Pogledajte [Run Detail View](#run-detail-view).
- **EU DSA član 17.** U EU regiji, potpuno automatizirane zabrane su blokirane. Pogledajte [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Nema treniranja na vašim podacima.** FastComments koristi provajdere koji ne treniraju na vašim promptovima ili komentarima.

### Gdje se uklapaju uz ljudsku moderaciju

Agenti i ljudski moderatori dijele istu platformu za komentare: agenti vrše radnje kroz iste kanale (odobri, označi kao spam, zabrani, dodijeli značku, prikvači, zaključaj, napiši) i te radnje se pojavljuju u istim [Dnevnicima komentara](/guide-moderation.html#comment-logs), na istoj [Stranici za moderaciju](/guide-moderation.html#moderate-comments-page) i u istim tokovima obavijesti. Agenti i ljudi vide tuđi rad i mogu međusobno reagovati — moderatorske radnje su same po sebi validni okidači za agente (pogledajte [Okidač: Moderator pregledao komentar](#trigger-moderator-reviewed) i slični).