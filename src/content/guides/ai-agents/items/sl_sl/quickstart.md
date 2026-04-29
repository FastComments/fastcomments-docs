To je petminutna pot od "imamo AI agente" do "agent odgovarja na promet v živo, omejen s soglasji." Če želite podrobnejšo različico, vsak korak vodi na stran, ki ga obravnava bolj poglobljeno.

### 1. Open the AI Agents page

Pojdite na [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) v svojem računu. Ob prvem obisku boste videli bodisi:

- Prazen zaslon z gumboma **Browse templates** in **Start from scratch** (na voljo imate agente za ustvarjanje), ali
- Stran z upsell ponudbo, če vaš paket ne vključuje agentov - glejte [Plans and Eligibility](#plans-and-eligibility).

### 2. Pick a starter template

Kliknite **Browse templates**. Izberite eno od:

- [Moderator](#template-moderator) - pregleda označene ali nove komentarje, prvič storilce opozori, za prepoved eskalira šele po opozorilu.
- [Welcome Greeter](#template-welcome-greeter) - odgovarja prvopristopnim komentatorjem.
- [Top Comment Pinner](#template-top-comment-pinner) - pripne vsebinske komentarje, ko presežejo prag glasov.
- [Thread Summarizer](#template-thread-summarizer) - objavi nevtralen povzetek pri dolgih nitih.

Vsaka predloga odpre vnaprej izpolnjen obrazec za urejanje z že izbrano možnostjo **Status: Dry Run**.

### 3. Review and save

Na obrazcu za urejanje naredite vsaj naslednje:

- **Internal name.** Kratek identifikator, uporabljen v skrbniških nadzornih ploščah.
- **Display name.** Kar se prikaže javno, ko agent objavi komentar.
- **Initial prompt.** Uredite poziv predloge tako, da ustreza vašemu tonu in vašim specifičnim pravilom.
- **Approvals.** Označite dejanja, ki naj zahtevajo človeški pregled, preden začnejo veljati. Priporočamo vsaj `ban_user` za kakršnega koli agenta za moderiranje. Oglejte si [Approval Workflow](#approval-workflow).

Kliknite **Save agent**.

### 4. Watch it in dry-run

Agent je zdaj aktiven v **Preizkusnem načinu**. Prejel bo sprožilce, poklical model in zabeležil dejanja na strani [Run History](#run-history) - z oznako **Preizkusni način** na vsaki vrstici - vendar ne izvaja dejanskih ukrepov. Oglejte si nekaj podrobnosti zagona (glejte [Run Detail View](#run-detail-view)) in preverite:

- Dejanja, ki jih je agent izbral.
- Utemeljitev in stopnjo zaupanja pri vsakem dejanju.
- Celoten LLM prepis.

Če agent sprejema odločitve, s katerimi se ne strinjate, uredite začetni poziv ali označite več odobritev.

### 5. Run a test against past comments

Na strani s seznamom agentov kliknite **Test run** na vrstici agenta. Obrazec ima en numerični vnos **Days** (1 do 90). Velikost vzorca in zgornja meja ocenjenih komentarjev sta prikazana informativno - izračunata se na strežniški strani, ne ju nastavlja uporabnik. Ponovitev izvaja test na zgodovinskih komentarjih brez izvajanja dejanskih ukrepov in poroča, kaj bi agent **storil** v primerjavi s tem, kar se je dejansko zgodilo (ali je bil komentar kasneje odobren, označen kot neželena pošta, izbrisan itd.). Oglejte si [Test Runs (Replays)](#test-runs-replays).

### 6. Flip to Enabled

Ko ste zadovoljni s preizkusnim zagonom in rezultati ponovitve, uredite agenta in spremenite **Status** v **Omogočeno**. Od tega trenutka naprej se izvajajo dejanski ukrepi. Stran Run History zdaj prikazuje žive zagone brez oznake preizkusnega načina, in vsako dejanje, ki ste ga označili za odobritev, se pojavi v [approvals inbox](#approval-workflow).

### What's next

- Nastavite [Budgets](#budgets-overview) in [Budget Alerts](#budget-alerts).
- Konfigurirajte [Webhooks](#webhooks-overview), če želite, da zunanji sistemi reagirajo na dogodke agenta.
- Dodajte [Community Guidelines](#community-guidelines), da bodo odločitve agenta usklajene z vašo pisno politiko.

---