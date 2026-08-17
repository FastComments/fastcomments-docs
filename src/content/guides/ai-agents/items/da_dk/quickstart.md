This is the five-minute path from "we have AI Agents" to "an agent is responding to live traffic, gated by approvals." If you want the long form, every step links to the page that covers it in depth.

### 1. Åbn siden AI‑agenter

Gå til [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) i din konto. Første gang du lander her, vil du se enten:

- En tom tilstand med en **Browse templates**‑ og **Start from scratch**‑knap (du har agenter klar til at blive oprettet), eller
- En opgraderingsside, hvis din plan ikke inkluderer agenter – se [Plans and Eligibility](#plans-and-eligibility).

### 2. Vælg en startskabelon

Klik på **Browse templates**. Vælg en af:

- [Moderator](#template-moderator) – gennemgår flagrede eller nye kommentarer, advarer førstegangs‑kommentatorer, eskalerer til ban kun efter en advarsel.
- [Welcome Greeter](#template-welcome-greeter) – svarer på førstegangs‑kommentatorer.
- [Top Comment Pinner](#template-top-comment-pinner) – fastgør væsentlige kommentarer, når de når en stemmetærskel.
- [Thread Summarizer](#template-thread-summarizer) – poster et neutralt resumé på lange tråde.

Each template lands on a pre-filled edit form with **Status: Dry Run** already selected.

### 3. Gennemgå og gem

On the edit form, do at minimum:

- **Internal name.** Et kort id, der bruges i admin‑dashboards.
- **Display name.** Det, der vises offentligt, når agenten poster en kommentar.
- **Initial prompt.** Rediger skabelonens prompt, så den passer til din tone og dine specifikke regler.
- **Approvals.** Marker de handlinger, der skal kræve menneskelig gennemgang, før de træder i kraft. Vi anbefaler mindst `ban_user` for enhver moderations‑agt. Se [Approval Workflow](#approval-workflow).

Click **Save agent**.

### 4. Se den i dry‑run

Agenten er nu aktiv i **Dry Run**. Den vil modtage sine triggere, kalde modellen og registrere handlinger på siden [Run History](#run-history) – med **Dry Run**‑badge på hver række – men den udfører ingen reelle handlinger. Besøg nogle af kørselens detaljer (se [Run Detail View](#run-detail-view)) og se på:

- De handlinger, agenten valgte.
- Begrundelsen og sikkerheden for hver handling.
- Den fulde LLM‑transkript.

If the agent is making decisions you disagree with, edit the initial prompt or tick more approvals.

### 5. Kør en test mod tidligere kommentarer

From the agents list page, click **Test run** on the agent's row. The form has a single **Days** numeric input (1 to 90). Sample size and the hard cap on comments evaluated are shown informationally - they are computed server-side, not user-set. The replay runs against historical comments without taking real actions and reports what the agent **would** have done versus what actually happened (was the comment later approved, marked spam, deleted, and so on). See [Test Runs (Replays)](#test-runs-replays).

### 6. Skift til Enabled

When you are happy with the dry-run and replay output, edit the agent and change **Status** to **Enabled**. From here on, real actions land. The Run History page now shows live runs without the dry-run badge, and any action you marked for approval appears in the [approvals inbox](#approval-workflow).

### Hvad er næste skridt

- Indstil [Budgets](#budgets-overview) og [Budget Alerts](#budget-alerts).
- Konfigurer [Webhooks](#webhooks-overview), hvis du vil have eksterne systemer til at reagere på agent‑begivenheder.
- Tilføj [Community Guidelines](#community-guidelines) for at holde agentens beslutninger i overensstemmelse med din skriftlige politik.