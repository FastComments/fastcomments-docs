Ovo je petominutni put od "we have AI Agents" do "an agent is responding to live traffic, gated by approvals." Ako želite detaljniju verziju, svaki korak vodi na stranicu koja ga pokriva detaljno.

### 1. Open the AI Agents page

Idite na [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) u vašem nalogu. Prvi put kada dođete ovde videćete ili:

- Prazno stanje sa **Browse templates** i **Start from scratch** dugmetima (imate agente koje možete kreirati), ili
- Stranicu za nadogradnju ako vaš plan ne uključuje agente - pogledajte [Plans and Eligibility](#plans-and-eligibility).

### 2. Pick a starter template

Kliknite **Browse templates**. Izaberite jedan od:

- [Moderator](#template-moderator) - pregleda označene ili nove komentare, upozorava one koji prvi put komentarišu, eskalira do zabrane tek posle upozorenja.
- [Welcome Greeter](#template-welcome-greeter) - odgovara onima koji prvi put komentarišu.
- [Top Comment Pinner](#template-top-comment-pinner) - prikači sadržajne komentare kada pređu prag glasova.
- [Thread Summarizer](#template-thread-summarizer) - postavlja neutralan rezime na dugim nitima.

Svaki predložak otvara unapred popunjen obrazac za uređivanje sa već izabranim **Status: Dry Run**.

### 3. Review and save

Na obrascu za uređivanje, uradite bar sledeće:

- **Internal name.** Kratki identifikator koji se koristi u administratorskim kontrolnim panelima.
- **Display name.** Ono što se javno prikazuje kada agent objavi komentar.
- **Initial prompt.** Izmenite prompt predloška da odgovara vašem tonu i vašim specifičnim pravilima.
- **Approvals.** Označite radnje koje bi trebalo da zahtevaju ljudski pregled pre nego što stupe na snagu. Preporučujemo bar `ban_user` za svaki agent za moderaciju. Pogledajte [Approval Workflow](#approval-workflow).

Kliknite **Save agent**.

### 4. Watch it in dry-run

Agent je sada aktivan u **Dry Run**. Primaće svoje okidače, pozivaće model i beležiti akcije na stranici [Run History](#run-history) - sa bedžom **Dry Run** u svakom redu - ali neće preduzimati stvarne akcije. Posetite nekoliko detalja pokretanja (pogledajte [Run Detail View](#run-detail-view)) i obratite pažnju na:

- Akcije koje je agent odabrao.
- Obrazloženje i nivo poverenja za svaku akciju.
- Potpuni LLM transkript.

Ako agent donosi odluke sa kojima se ne slažete, izmenite početni prompt ili označite više odobrenja.

### 5. Run a test against past comments

Sa stranice sa listom agenata, kliknite **Test run** na redu agenta. Forma ima jedno numeričko polje **Days** (1 do 90). Veličina uzorka i maksimalni broj komentara koji se ocenjuju prikazuju se informativno — računaju se na serverskoj strani, ne postavlja ih korisnik. Replay se pokreće nad istorijskim komentarima bez preduzimanja stvarnih akcija i izveštava šta bi agent **uradio** u odnosu na ono što se zapravo desilo (da li je komentar kasnije odobren, označen kao spam, obrisan itd.). Pogledajte [Test Runs (Replays)](#test-runs-replays).

### 6. Flip to Enabled

Kada ste zadovoljni izlazom iz dry-run i replay-a, izmenite agenta i promenite **Status** u **Enabled**. Od tog trenutka dalje, stvarne akcije se izvršavaju. Stranica [Run History](#run-history) sada prikazuje užive pokrete bez bedža za dry-run, i svaka akcija koju ste označili za odobrenje pojavljuje se u [approvals inbox](#approval-workflow).

### Šta sledi

- Podesite [Budgets](#budgets-overview) i [Budget Alerts](#budget-alerts).
- Konfigurišite [Webhooks](#webhooks-overview) ako želite da eksterni sistemi reaguju na događaje agenata.
- Dodajte [Community Guidelines](#community-guidelines) da odluke agenta budu usklađene sa vašom pisanim politikom.