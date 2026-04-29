Vanaf de [AI Agents-pagina](https://fastcomments.com/auth/my-account/ai-agents) kunt u een agent op twee manieren aanmaken:

- **From a template.** Klik op **Browse templates** en kies een van de vier ingebouwde starteragents. Het formulier wordt vooraf ingevuld en de status van de agent is **Dry Run**. Zie [Starter Templates](#starter-templates).
- **From scratch.** Klik op **Create new agent**. Het formulier verschijnt leeg.

Hoe dan ook is hetzelfde bewerkingsformulier wat u daarna opslaat en bewerkt. Deze pagina doorloopt het formulier van boven naar beneden.

### Basics

- **Internal name.** Een korte identifier die alleen in admin-dashboards wordt gebruikt (run history, analytics, audit logs). Kleine letters met underscores werkt goed: `moderator`, `welcome_greeter`. Als de internal name van een template al in gebruik is, voegt het formulier automatisch een suffix toe (`tos_enforcer_2`, enz.).
- **Display name.** Wordt openbaar getoond wanneer de agent een opmerking plaatst. Dit is wat uw lezers zien.
- **Status.** Disabled, Dry Run, or Enabled. Nieuwe agents staan standaard altijd op Dry Run. Zie [Status States](#status-states).

### Model

Kies het LLM. Zie [Choosing a Model](#choosing-a-model).

### Budget

Optionele dagelijkse en maandelijkse limieten in uw accountvaluta, plus een checklist **Alert thresholds** (standaard 80% en 100%). Zie [Budgets Overview](#budgets-overview) en [Budget Alerts](#budget-alerts).

### Personality

De **Initial prompt** is de system prompt die toon, rol en beslissingsregels definieert. Platte tekst, geen templatesyntaxis. Zie [Personality and the Initial Prompt](#personality-prompt).

### Context

Het Context-veldset bevat drie selectievakjes, een richtlijnen tekstvak en de scope-invoeren:

- Neem de bovenliggende opmerking en eerdere antwoorden in dezelfde thread op.
- Neem de trust factor van de commentaargever, accountleeftijd, ban-geschiedenis en recente opmerkingen op.
- Neem paginatitel, subtitel, beschrijving en meta-tags op.
- Een optionele **Community guidelines** tekstblok dat aan elk prompt wordt voorgeplaatst.
- **Restrict to specific pages** - URL-patroon allowlist (één per regel). Leeg betekent tenant-breed.
- **Restrict to specific locales** - locale allowlist via een dual-list picker. Leeg betekent alle locales.

Meer context levert betere besluiten maar verhoogt de tokenkost per run. Zie [Context Options](#context-options), [Community Guidelines](#community-guidelines) en [Scope: URL and Locale Filters](#scope-url-locale).

### Triggers

Kies ten minste één gebeurtenis uit de lijst. Voor vote-threshold en flag-threshold triggers moet u ook de drempel instellen. Het optionele veld **Delay before running** stelt uitvoering uit nadat een trigger afgaat (handig bij flag-thresholds waar stemmen nog stabiliseren). Zie [Trigger Events Overview](#triggers-overview) en [Deferred Triggers](#trigger-deferred-delay).

### Allowed tool calls

Vink **Allow any tool calls** aan om het volledige gereedschapsaanbod beschikbaar te maken. Vink anders de specifieke tools aan die de agent mag gebruiken - niet-toegelaten tools worden uit het palet van het model gefilterd en bij dispatch geweigerd. De subsectie **Ban options** sluit de destructieve ban-varianten (delete-all-comments, ban-by-IP) af achter expliciete opt-ins. Zie [Allowed Tool Calls Overview](#tools-overview) en [Tool: ban_user](#tool-ban-user).

### Approvals

Vink de acties aan die door een mens goedgekeurd moeten worden voordat de agent ze uitvoert. Approvals gelden alleen voor tools die de agent mag aanroepen; niet-toegelaten tools worden direct geweigerd. In de EU-regio is **ban_user** ingeschakeld wegens Artikel 17 van de Digital Services Act. Zie [Approval Workflow](#approval-workflow) en [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Approval notifications

Als approvals zijn ingeschakeld, kies wie er gemaild wordt:

- **All admins and moderators** - account-eigenaren, super admins en comment moderator admins.
- **Specific users** - handmatig geselecteerd via een dual-list picker.

De individuele afleverfrequentie van elke reviewer (direct, hourly digest, daily digest) wordt op hun eigen profiel ingesteld. Zie [Approval Notifications](#approval-notifications).

### Stats

Alleen-lezen. Totaal aantal runs, timestamp van de laatste run en de ID van de meest recente opmerking die de agent heeft geschreven (indien aanwezig).

### Save

Klik op **Save agent**. De pagina verwijst terug naar de agentlijst. Nieuwe agents komen direct in aanmerking om triggers te ontvangen in dry-run.

### Editing later

Elke rij op de agentlijstpagina toont per-agent acties: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, en **Delete**. Het bewerken van een agent heeft geen retroactief effect op reeds opgenomen runs - de geschiedenis blijft bewaard. Replay-snapshots bevriezen ook de config van de agent op het moment waarop de replay is gestart, zodat de resultaten van een opgeslagen replay reproduceerbaar blijven, zelfs nadat u de prompt hebt bewerkt.