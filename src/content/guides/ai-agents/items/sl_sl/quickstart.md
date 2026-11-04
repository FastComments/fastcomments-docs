To je petminutna pot od »imamo AI agente« do »agent odgovarja na živi promet, zavarovan z odobritvami.« Če želite podrobnejšo različico, se vsak korak povezuje s stranjo, ki to temo obsežno pokriva.

### 1. Odprite stran AI Agents

Pojdite na [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) v svojem računu. Ko prvič pridete sem, boste videli:

- Prazno stanje z gumboma **Browse templates** in **Start from scratch** (na voljo imate agente za ustvarjanje), ali
- Stran za nadgradnjo paketa, če vaš paket ne vključuje agentov - glejte [Plans and Eligibility](#plans-and-eligibility).

### 2. Izberite začetno predlogo

Kliknite **Browse templates**. Izberite eno od:

- [Moderator](#template-moderator) - pregleduje prijavljene ali nove komentarje, opozori prvočasnike, prepoved (ban) uvede šele po opozorilu.
- [Welcome Greeter](#template-welcome-greeter) - odgovarja prvočasnim komentatorjem.
- [Top Comment Pinner](#template-top-comment-pinner) - pripne vsebinske komentarje, ko prečkajo prag glasov.
- [Thread Summarizer](#template-thread-summarizer) - objavi nevtralni povzetek na dolgih nitih.

Vsaka predloga odpre vnaprej izpolnjen obrazec za urejanje, kjer je že izbran **Status: Dry Run**.

### 3. Preglejte in shranite

Na obrazcu za urejanje naredite vsaj naslednje:

- **Internal name.** Kratek identifikator, uporabljen v skrbniških nadzornih ploščah.
- **Display name.** Kaj se javno prikaže, ko agent objavi komentar.
- **Initial prompt.** Uredite poziv predloge, da ustreza vašemu tonu in vašim specifičnim pravilom.
- **Approvals.** Označite dejanja, ki naj zahtevajo človeški pregled, preden začnejo veljati. Priporočamo vsaj `ban_user` za vsakega agenta za moderiranje. Glejte [Approval Workflow](#approval-workflow).

Kliknite **Save agent**.

### 4. Spremljajte ga v poskusnem načinu

Agent je zdaj aktiven v načinu **Dry Run**. Prejema svoje sprožilce, kliče model in beleži dejanja na strani [Run History](#run-history) - z oznako **Dry Run** v vsaki vrstici - vendar ne izvaja pravih dejanj. Oglejte si nekaj podrobnosti izvajanj (glejte [Run Detail View](#run-detail-view)) in poglejte:

- Dejanja, ki jih je agent izbral.
- Utemeljitev in zaupanje pri vsakem dejanju.
- Celoten prepis LLM.

Če se agent odloča na način, s katerim se ne strinjate, uredite začetni poziv ali označite več odobritev.

### 5. Izvedite test na preteklih komentarjih

Na strani s seznamom agentov kliknite **Test run** v vrstici agenta. Obrazec ima eno številsko polje **Days** (1 do 90). Velikost vzorca in trdi omejitev ocenjenih komentarjev sta prikazana informativno - izračunana sta na strežniku, ne nastavljena s strani uporabnika. Ponovitev se izvede na zgodovinskih komentarjih brez pravih dejanj in poroča, kaj bi agent **naredil**, v primerjavi s tem, kaj se je dejansko zgodilo (ali je bil komentar kasneje odobren, označen kot spam, izbrisan itd.). Glejte [Test Runs (Replays)](#test-runs-replays).

### 6. Preklopite na Enabled

Ko ste zadovoljni z izhodom poskusnega načina in ponovitve, uredite agenta in spremenite **Status** na **Enabled**. Od tu naprej se izvajajo prava dejanja. Stran Run History zdaj prikazuje živa izvajanja brez oznake poskusnega načina, vsako dejanje, označeno za odobritev, pa se pojavi v [vhodnem predalu odobritev](#approval-workflow).

### Kaj še

- Nastavite [Budgets](#budgets-overview) in [Budget Alerts](#budget-alerts).
- Konfigurirajte [Webhooks](#webhooks-overview), če želite, da zunanji sistemi reagirajo na dogodke agentov.
- Dodajte [Community Guidelines](#community-guidelines), da ostanejo odločitve agentov usklajene z vašo pisno politiko.