An **AI Agent** je autonomni radnik, ograničen na vaš FastComments tenant, koji prati događaje u vašoj zajednici i preduzima akcije u vaše ime.

Svaki agent ima tri stvari koje možete kontrolisati:

1. **A personality.** Slobodni tekst koji definiše ton, ulogu i stil donošenja odluka ("You are a warm community greeter", "You enforce the community rules but err toward warning over banning", i slično).
2. **One or more triggers.** Lista događaja koji bude agenta - novi komentar, komentar koji prelazi prag glasova ili prijava, moderatorska akcija, prvi komentar korisnika na sajtu i drugi. Cela lista je u [Pregled događaja okidača](#triggers-overview).
3. **An allowlist of tools.** Šta je agentu dozvoljeno da radi - objaviti komentar, glasati, zakačiti, zaključati, označiti kao spam, zabraniti korisnika, upozoriti preko DM, dodeliti značku, poslati email, sačuvati i pretražiti deljenu memoriju. Cela lista je u [Pregled dozvoljenih poziva alata](#tools-overview).

Kada se okidač aktivira, agent prima poruku sa kontekstom koja opisuje šta se dogodilo (komentar, stranica, opcioni kontekst niti/korisnika/stranice) i dobija svoj početni prompt i smernice vaše zajednice. Zatim poziva alate da deluje, beležeći opravdanje i ocenu poverenja za svaki poziv.

### Agenti rade asinhrono

Agenti **nikada ne blokiraju korisničku radnju koja ih je pokrenula**. Čitalac objavi komentar, komentar se sačuva i prikaže u niti, odgovor se vrati, i tek *tada* agent na njega izvršava - bilo odmah ili nakon konfigurisanog kašnjenja (vidi [Odloženi okidači](#trigger-deferred-delay)). Ništa što agent radi ne dodaje latenciju iskustvu koje vidi korisnik.

### Zašto ih koristiti

- **Moderirajte u velikom obimu.** Obeležite očigledan spam i zabranite ponavljače bez stalnog nadgledanja reda.
- **Pozdravljajte nove komentatore.** Odgovarajte prvim komentatorima u vašem tonu.
- **Istaknite najbolji sadržaj.** Zakačite suštinske komentare prvog nivoa kada pređu prag glasova.
- **Dosledno sprovodite smernice.** Primenu iste politike na svaki granicni komentar.
- **Sumirajte duge niti.** Objavite neutralne sažetke debata na više stranica.

### Šta vam daje kontrolu

- **Režim suve provere.** Svaki novi agent se isporučuje u režimu **Dry Run**: procesuira okidače, pokreće model i beleži šta bi uradio, ali ne preduzima stvarne akcije. Vidi [Režim suve provere](#dry-run-mode).
- **Odobrenja.** Bilo koji podskup akcija može biti ograničen ljudskim odobravanjem. Vidi [Tok odobravanja](#approval-workflow).
- **Budžeti po agentu i računu.** Strogi dnevni i mesečni limiti. Vidi [Pregled budžeta](#budgets-overview).
- **Lista dozvoljenih alata.** Nedozvoljeni alati se uklanjaju iz palete modela - agent ih bukvalno ne može zatražiti. Vidi [Pregled dozvoljenih poziva alata](#tools-overview).
- **Polja revizije za svaku akciju.** Model mora uključiti opravdanje i ocenu poverenja. Obe se pojavljuju u vremenskoj liniji izvršavanja i na svakom odobrenju. Vidi [Prikaz detalja izvršavanja](#run-detail-view).
- **EU DSA Article 17.** U regionu EU, potpuno automatizovane zabrane su blokirane. Vidi [Usklađenost sa EU DSA čl. 17](#eu-dsa-compliance).
- **Bez treniranja na vašim podacima.** FastComments koristi provajdere koji ne treniraju na vašim promptovima ili komentarima.

### Gde se uklapaju uz ljudsku moderaciju

Agenti i ljudski moderatori dele istu platformu za komentare: agenti preduzimaju akcije putem istih kanala (odobri, označi kao spam, zabrani, dodeli značku, zakači, zaključaj, napiši) i te akcije se pojavljuju u istim [Dnevnicima komentara](/guide-moderation.html#comment-logs), istoj [Stranici za moderaciju](/guide-moderation.html#moderate-comments-page) i istim tokovima obaveštenja. Agenti i ljudi vide rad jedni drugih i mogu međusobno reagovati - moderatorske akcije su same po sebi važeći okidači za agente (vidi [Okidač: Moderator pregledao komentar](#trigger-moderator-reviewed) i slični).