Ovo je petominutni put od "imamo AI agente" do "agent odgovara na promet uživo, uz odobrenja." Ako želite dugačku verziju, svaki korak povezuje stranicu koja ga detaljno objašnjava.

### 1. Otvorite stranicu AI agenata

Idite na [AI agenata](https://fastcomments.com/auth/my-account/ai-agents) u svom računu. Prvi put kad dođete ovdje vidjet ćete ili:

- Stanje bez sadržaja s gumbima **Pregled predložaka** i **Počni od početka** (imati ćete agente koje možete kreirati), ili
- stranicu za nadograđivanje ako vaš plan ne uključuje agente - pogledajte [Planovi i podobnost](#plans-and-eligibility).

### 2. Odaberite početni predložak

Kliknite **Pregled predložaka**. Odaberite jedan od:

- [Moderator](#template-moderator) - pregledava označene ili nove komentare, upozorava komentatore koji prvi put komentiraju, i eskalira do zabrane tek nakon upozorenja.
- [Welcome Greeter](#template-welcome-greeter) - odgovara komentatorima koji komentiraju prvi put.
- [Top Comment Pinner](#template-top-comment-pinner) - prikvači sadržajne komentare nakon što prijeđu prag glasova.
- [Thread Summarizer](#template-thread-summarizer) - objavljuje neutralan sažetak na dugim nitima.

Svaki predložak otvara unaprijed ispunjeni obrazac za uređivanje s već odabranim **Status: Probni način**.

### 3. Pregledajte i spremite

Na obrascu za uređivanje, barem uredite:

- **Internal name.** Kratki identifikator koji se koristi u administratorskim nadzornim pločama.
- **Display name.** Ono što se javno prikazuje kad agent objavi komentar.
- **Initial prompt.** Uredite početni upit predloška da odgovara vašem tonu i vašim specifičnim pravilima.
- **Approvals.** Označite radnje koje bi trebale zahtijevati ljudski pregled prije nego što stupe na snagu. Preporučujemo barem `ban_user` za bilo koji agent u stilu moderiranja. Pogledajte [Tijek odobravanja](#approval-workflow).

Kliknite **Spremi agenta**.

### 4. Promatrajte u probnom načinu

Agent je sada aktivan u **Probnom načinu**. Primat će svoje okidače, pozivati model i bilježiti radnje na stranici [Povijest izvršavanja](#run-history) - s oznakom **Probni način** na svakom retku - ali ne izvršava stvarne radnje. Posjetite nekoliko detalja pokretanja (pogledajte [Pregled detalja pokretanja](#run-detail-view)) i pogledajte:

- Radnje koje je agent odabrao.
- Opravdanje i povjerenje u svaku radnju.
- Potpuni LLM transkript.

Ako agent donosi odluke s kojima se ne slažete, uredite početni upit ili označite više odobrenja.

### 5. Pokrenite test na prošlim komentarima

S popisa agenata, kliknite **Test run** na retku agenta. Obrazac ima jedino numeričko polje **Days** (1 do 90). Veličina uzorka i gornja granica za komentare koji se vrednuju prikazani su informativno - izračunavaju se na strani poslužitelja, a ne postavlja ih korisnik. Replay se pokreće na povijesnim komentarima bez poduzimanja stvarnih radnji i izvještava što bi agent **učinio** nasuprot onome što se zapravo dogodilo (je li komentar kasnije odobren, označen kao spam, izbrisan itd.). Pogledajte [Testni pokusi (reprodukcije)](#test-runs-replays).

### 6. Prebacite na Omogućeno

Kada ste zadovoljni rezultatima probnog načina i reprodukcije, uredite agenta i promijenite **Status** u **Omogućeno**. Od ovog trenutka, stvarne radnje se izvršavaju. Stranica Povijest izvršavanja sada prikazuje žive pokrete bez oznake probnog načina, a svaka radnja koju ste označili za odobrenje pojavljuje se u [inboxu za odobrenja](#approval-workflow).

### Što dalje

- Postavite [Budžete](#budgets-overview) i [Upozorenja o budžetu](#budget-alerts).
- Konfigurirajte [Webhookove](#webhooks-overview) ako želite da vanjski sustavi reagiraju na događaje agenata.
- Dodajte [Smjernice zajednice](#community-guidelines) kako biste održali odluke agenata u skladu s vašom pisanim pravilima.