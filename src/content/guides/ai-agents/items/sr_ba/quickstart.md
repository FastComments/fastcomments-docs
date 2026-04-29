Ovo je petominutni put od "imamo AI agente" do "agent odgovara na uživi saobraćaj, uz odobrenja." Ako želite detaljniju verziju, svaki korak povezuje na stranicu koja ga detaljno pokriva.

### 1. Open the AI Agents page

Idite na [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) u svom nalogu. Prvi put kada stignete ovdje vidjećete ili:

- Stanje bez sadržaja sa dugmetima **Browse templates** i **Start from scratch** (imate agente dostupne za kreiranje), ili
- Stranicu za nadogradnju ako vaš plan ne uključuje agente - vidite [Plans and Eligibility](#plans-and-eligibility).

### 2. Pick a starter template

Kliknite **Browse templates**. Odaberite jedan od:

- [Moderator](#template-moderator) - pregledava označene ili nove komentare, upozorava korisnike koji komentarišu prvi put, i eskalira na zabranu tek nakon upozorenja.
- [Welcome Greeter](#template-welcome-greeter) - odgovara korisnicima koji komentarišu prvi put.
- [Top Comment Pinner](#template-top-comment-pinner) - pričvršćuje sadržajne komentare kada pređu prag glasova.
- [Thread Summarizer](#template-thread-summarizer) - objavljuje neutralni sažetak na dugim nitima.

Svaki predložak otvara predpopunjeni obrazac za uređivanje sa već odabranim **Status: Probno izvođenje**.

### 3. Review and save

Na obrascu za uređivanje, uradite najmanje:

- **Internal name.** Kratki identifikator koji se koristi u administrativnim kontrolnim tablama.
- **Display name.** Ono što se javno prikazuje kada agent objavi komentar.
- **Initial prompt.** Uredite prompt iz predloška da odgovara vašem tonu i vašim specifičnim pravilima.
- **Approvals.** Označite akcije koje trebaju ljudsku provjeru prije nego stupe na snagu. Preporučujemo barem `ban_user` za bilo kojeg agenta za moderaciju. Pogledajte [Tok odobravanja](#approval-workflow).

Kliknite **Sačuvaj agenta**.

### 4. Watch it in dry-run

Agent je sada aktivan u okviru **Probnog izvođenja**. Prihvata svoje okidače, poziva model i zapisuje akcije na stranici [Run History](#run-history) - sa bedžom **Probnog izvođenja** na svakom redu - ali ne izvodi stvarne akcije. Posjetite nekoliko detalja izvršavanja (vidi [Pregled detalja izvršavanja](#run-detail-view)) i provjerite:

- Akcije koje je agent odabrao.
- Opravdanje i stepen povjerenja za svaku akciju.
- Potpuni LLM transkript.

Ako agent donosi odluke s kojima se ne slažete, uredite početni prompt ili označite više odobrenja.

### 5. Run a test against past comments

Sa stranice liste agenata, kliknite **Test run** na redu agenta. Obrazac ima jedan numerički unos **Days** (1 do 90). Veličina uzorka i maksimalni broj komentara koji se evaluiraju prikazani su informativno - izračunavaju se na serverskoj strani, ne postavlja ih korisnik. Replay pokreće se protiv historijskih komentara bez izvođenja stvarnih akcija i izveštava šta bi agent **bio** uradio naspram onoga što se zapravo desilo (da li je komentar kasnije odobren, označen kao spam, obrisan, i tako dalje). Vidi [Test Runs (Replays)](#test-runs-replays).

### 6. Flip to Enabled

Kada budete zadovoljni sa rezultatima probnog izvođenja i replay-a, uredite agenta i promijenite **Status** u **Enabled**. Od tog trenutka, stvarne akcije se izvršavaju. Stranica Run History sada prikazuje žive izvođenja bez bedža probnog izvođenja, i svaka akcija koju ste označili za odobrenje pojavljuje se u [approvals inbox](#approval-workflow).

### What's next

- Podesite [Budgets](#budgets-overview) i [Budget Alerts](#budget-alerts).
- Konfigurišite [Webhooks](#webhooks-overview) ako želite da eksterni sistemi reaguju na događaje agenata.
- Dodajte [Community Guidelines](#community-guidelines) kako biste odluke agenata uskladili sa vašom pisanim politikama.