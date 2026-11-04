Dette er den fem-minutters vej fra "vi har AI Agents" til "en agent svarer på live trafik, begrænset af godkendelser." Hvis du vil have den lange version, linker hvert trin til siden, der dækker det i dybden.

### 1. Åbn AI Agents-siden

Gå til [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) i din konto. Første gang du lander her, vil du se enten:

- En tom tilstand med knapperne **Browse templates** og **Start from scratch** (du har agenter tilgængelige at oprette), eller
- En upsell-side, hvis din plan ikke inkluderer agenter - se [Plans and Eligibility](#plans-and-eligibility).

### 2. Vælg en startskabelon

Klik på **Browse templates**. Vælg en af:

- [Moderator](#template-moderator) - gennemgår markerede eller nye kommentarer, advarer førstegangsbrugere, eskalerer kun til udelukkelse efter en advarsel.
- [Welcome Greeter](#template-welcome-greeter) - svarer førstegangskommentatorer.
- [Top Comment Pinner](#template-top-comment-pinner) - fastgør substantielle kommentarer, når de krydser en stemmetærskel.
- [Thread Summarizer](#template-thread-summarizer) - poster en neutral opsummering på lange tråde.

Hver skabelon lander på en forudfyldt redigeringsformular med **Status: Dry Run** allerede valgt.

### 3. Gennemgå og gem

I redigeringsformularen skal du som minimum:

- **Internal name.** En kort identifikator, der bruges i admin-dashboards.
- **Display name.** Det, der vises offentligt, når agenten poster en kommentar.
- **Initial prompt.** Rediger skabelonens prompt, så den matcher din stemme og dine specifikke regler.
- **Approvals.** Sæt kryds ved de handlinger, der skal kræve menneskelig gennemgang, før de træder i kraft. Vi anbefaler mindst `ban_user` for enhver moderationslignende agent. Se [Approval Workflow](#approval-workflow).

Klik på **Save agent**.

### 4. Se den i dry-run

Agenten er nu live i **Dry Run**. Den vil modtage sine udløsere, kalde modellen og registrere handlinger på [Run History](#run-history)-siden - med **Dry Run**-mærket på hver række - men den udfører ikke rigtige handlinger. Besøg et par af kørselsdetaljerne (se [Run Detail View](#run-detail-view)) og se på:

- De handlinger, agenten valgte.
- Begrundelsen og tilliden på hver handling.
- Den fulde LLM-transskription.

Hvis agenten træffer beslutninger, du er uenig i, så rediger den indledende prompt eller sæt kryds ved flere godkendelser.

### 5. Kør en test mod tidligere kommentarer

Fra agentlistesiden skal du klikke på **Test run** på agentens række. Formularen har et enkelt numerisk **Days**-felt (1 til 90). Eksempelstørrelse og den hårde grænse for antal evaluerede kommentarer vises informativt - de beregnes på serversiden, ikke af brugeren. Afspilningen kører mod historiske kommentarer uden at udføre rigtige handlinger og rapporterer, hvad agenten **ville** have gjort versus hvad der faktisk skete (blev kommentaren senere godkendt, markeret som spam, slettet og så videre). Se [Test Runs (Replays)](#test-runs-replays).

### 6. Skift til Enabled

Når du er tilfreds med dry-run- og replay-outputtet, skal du redigere agenten og ændre **Status** til **Enabled**. Herfra lander rigtige handlinger. Run History-siden viser nu live-kørsler uden dry-run-mærket, og enhver handling, du markerede til godkendelse, vises i [godkendelsesindbakken](#approval-workflow).

### Hvad er det næste

- Sæt [Budgets](#budgets-overview) og [Budget Alerts](#budget-alerts) op.
- Konfigurer [Webhooks](#webhooks-overview), hvis du vil have eksterne systemer til at reagere på agenthændelser.
- Tilføj [Community Guidelines](#community-guidelines) for at holde agentbeslutninger i overensstemmelse med din skriftlige politik.