Dette er fem-minuttersvejen fra "vi har AI-agenter" til "en agent besvarer live-trafik, styret af godkendelser." Hvis du vil have den lange version, linker hvert trin til siden, der beskriver det i dybden.

### 1. Open the AI Agents page

Gå til [AI-agenter](https://fastcomments.com/auth/my-account/ai-agents) i din konto. Første gang du lander her vil du se enten:

- En tom tilstand med en **Gennemse skabeloner** og **Start fra bunden** knapper (du har agenter tilgængelige til at oprette), eller
- en opsalgsside hvis din plan ikke inkluderer agenter - se [Planer og berettigelse](#plans-and-eligibility).

### 2. Pick a starter template

Klik på **Gennemse skabeloner**. Vælg en af:

- [Moderator](#template-moderator) - gennemgår markerede eller nye kommentarer, advarer førstegangsforfattere, eskalerer til udelukkelse kun efter en advarsel.
- [Velkomsthilsen](#template-welcome-greeter) - svarer til førstegangs-kommentatorer.
- [Fastgør topkommentarer](#template-top-comment-pinner) - fastgør væsentlige kommentarer, når de passerer en stemmegrænse.
- [Trådopsummerer](#template-thread-summarizer) - udgiver et neutralt sammendrag på lange tråde.

Hver skabelon åbner en forudfyldt redigeringsformular med **Status: Dry Run** allerede valgt.

### 3. Review and save

På redigeringsformularen, gør som minimum:

- **Internt navn.** En kort identifikator brugt i administrationspaneler.
- **Visningsnavn.** Det, der vises offentligt, når agenten skriver en kommentar.
- **Startprompt.** Rediger skabelonens prompt, så den matcher din tone og dine specifikke regler.
- **Godkendelser.** Sæt kryds ved de handlinger, der skal kræve menneskelig gennemgang, før de træder i kraft. Vi anbefaler mindst `ban_user` for enhver moderationsagent. Se [Godkendelsesworkflow](#approval-workflow).

Klik på **Gem agent**.

### 4. Watch it in dry-run

Agenten er nu aktiv i **Dry Run**. Den vil modtage sine triggere, kalde modellen og registrere handlinger på [Kørselshistorik](#run-history)-siden - med **Dry Run**-badgen på hver række - men den udfører ikke reelle handlinger. Besøg et par af kørselsdetaljerne (se [Kørselsdetaljevisning](#run-detail-view)) og kig på:

- De handlinger, agenten valgte.
- Begrundelsen og tilliden for hver handling.
- Den fulde LLM-transkription.

Hvis agenten træffer beslutninger, du er uenig i, rediger startprompten eller sæt kryds ved flere godkendelser.

### 5. Run a test against past comments

Fra agentlisten, klik på **Test run** på agentens række. Formularen har et enkelt numerisk input **Dage** (1 til 90). Prøvestørrelse og den faste øvre grænse for evaluerede kommentarer vises kun som information - de beregnes på serversiden, ikke af brugeren. Genspilsafviklingen kører mod historiske kommentarer uden at udføre reelle handlinger og rapporterer, hvad agenten **ville** have gjort i forhold til, hvad der faktisk skete (blev kommentaren senere godkendt, markeret som spam, slettet osv.). Se [Testkørsler (Genspil)](#test-runs-replays).

### 6. Flip to Enabled

Når du er tilfreds med Dry Run og gengivelsesresultaterne, rediger agenten og ændr **Status** til **Aktiveret**. Fremover udføres reelle handlinger. Siden [Kørselshistorik](#run-history) viser nu live-kørsler uden Dry Run-badgen, og enhver handling, du markerede til godkendelse, vises i [godkendelsesindbakken](#approval-workflow).

### What's next

- Indstil [Budgetter](#budgets-overview) og [Budgetadvarsler](#budget-alerts).
- Konfigurer [Webhooks](#webhooks-overview), hvis du ønsker, at eksterne systemer reagerer på agentbegivenheder.
- Tilføj [Fællesskabsretningslinjer](#community-guidelines) for at holde agentens beslutninger i overensstemmelse med din skriftlige politik.