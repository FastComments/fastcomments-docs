Ovo je pet‑minutni put od „imamo AI agente“ do „agent odgovara na stvarni saobraćaj, uz odobrenja“. Ako želite detaljniji opis, svaki korak vodi na stranicu koja ga detaljno obrađuje.

### 1. Otvorite stranicu AI Agents

Idite na [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) u vašem nalogu. Prvi put kada dođete ovde videćete jedno od sledećeg:

- Prazno stanje sa dugmadima **Browse templates** i **Start from scratch** (imajte agente spremne za kreiranje), ili
- Stranicu za nadogradnju ako vaš plan ne uključuje agente – pogledajte [Plans and Eligibility](#plans-and-eligibility).

### 2. Izaberite početni šablon

Kliknite **Browse templates**. Izaberite jedan od:

- [Moderator](#template-moderator) – pregledava označene ili nove komentare, upozorava nove korisnike, eskalira do zabrane tek nakon upozorenja.
- [Welcome Greeter](#template-welcome-greeter) – odgovara na komentare prvih posetilaca.
- [Top Comment Pinner](#template-top-comment-pinner) – zakači značajne komentare kada pređu prag glasova.
- [Thread Summarizer](#template-thread-summarizer) – objavljuje neutralni rezime na dugim temama.

Svaki šablon otvara unapred popunjen obrazac za uređivanje sa već izabranim **Status: Dry Run**.

### 3. Pregledajte i sačuvajte

Na obrascu za uređivanje, uradite bar sledeće:

- **Internal name.** Kratki identifikator koji se koristi u administratorskim kontrolnim tablama.
- **Display name.** Kako se agent prikazuje javno kada objavi komentar.
- **Initial prompt.** Izmenite prompt šablona da odgovara vašem tonu i specifičnim pravilima.
- **Approvals.** Označite radnje koje treba da zahtevaju ljudsku reviziju pre nego što stupe na snagu. Preporučujemo najmanje `ban_user` za bilo kog agenta koji moderira. Pogledajte [Approval Workflow](#approval-workflow).

Kliknite **Save agent**.

### 4. Posmatrajte ga u režimu suve probe

Agent je sada aktivan u **Dry Run** režimu. Prima svoje okidače, poziva model i beleži radnje na stranici [Run History](#run-history) – sa oznakom **Dry Run** na svakom redu – ali ne preduzima stvarne radnje. Posetite nekoliko detalja pokretanja (vidite [Run Detail View](#run-detail-view)) i pogledajte:

- Radnje koje je agent odabrao.
- Obrazloženje i pouzdanost za svaku radnju.
- Potpunu LLM transkripciju.

Ako agent donosi odluke sa kojima se ne slažete, izmenite početni prompt ili označite više odobrenja.

### 5. Pokrenite test na prošlim komentarima

Sa stranice spiska agenata, kliknite **Test run** na redu agenta. Obrazac ima jedno numeričko polje **Days** (1 do 90). Veličina uzorka i maksimalan broj komentara koji se ocenjuju prikazani su informativno – izračunavaju se na serveru, a ne postavljaju od strane korisnika. Reprodukcija se izvršava na istorijskim komentarima bez preduzimanja stvarnih radnji i izveštava šta bi agent **uradio**, naspram onoga što se zaista dogodilo (da li je komentar kasnije odobren, označen kao spam, obrisan, itd.). Pogledajte [Test Runs (Replays)](#test-runs-replays).

### 6. Prebacite na Enabled

Kada budete zadovoljni rezultatima suve probe i reprodukcije, izmenite agenta i promenite **Status** u **Enabled**. Od tog trenutka stvarne radnje se primenjuju. Stranica Run History sada prikazuje žive pokretanja bez oznake suve probe, a svaka radnja koju ste označili za odobrenje pojavljuje se u [approvals inbox](#approval-workflow).

### Šta sledi

- Postavite [Budgets](#budgets-overview) i [Budget Alerts](#budget-alerts).
- Konfigurišite [Webhooks](#webhooks-overview) ako želite da eksterne sisteme obavestite o događajima agenta.
- Dodajte [Community Guidelines](#community-guidelines) kako bi odluke agenta bile u skladu sa vašom pisanom politikom.