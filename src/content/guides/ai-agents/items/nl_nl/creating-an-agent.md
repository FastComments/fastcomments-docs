Van de [AI Agents-pagina](https://fastcomments.com/auth/my-account/ai-agents) kun je op twee manieren een agent aanmaken:

- **Vanuit een template.** Klik op **Browse templates** en kies een van de vier ingebouwde starter-agents. Het formulier wordt vooringevuld geopend en de status van de agent is **Dry Run**. Zie [Starter Templates](#starter-templates).
- **Vanaf nul.** Klik op **Create new agent**. Het formulier wordt leeg geopend.

In beide gevallen gebruik je daarna hetzelfde bewerkingsformulier om op te slaan en te wijzigen. Deze pagina doorloopt het formulier van boven naar beneden.

### Basics

- **Internal name.** Een korte identificatie die alleen in admin-dashboards wordt gebruikt (run-geschiedenis, analytics, auditlogs). Kleine letters met underscores werken goed: `moderator`, `welcome_greeter`. Als de internal name van een template al in gebruik is, voegt het formulier automatisch een suffix toe (`tos_enforcer_2`, enz.).
- **Display name.** Wordt openbaar getoond wanneer de agent een reactie plaatst. Dit is wat je lezers zien.
- **Status.** Disabled, Dry Run of Enabled. Nieuwe agents staan standaard op Dry Run. Zie [Status States](#status-states).

### Model

Kies het LLM. Zie [Choosing a Model](#choosing-a-model).

### Budget

Optionele dagelijkse en maandelijkse limieten in de valuta van je account, plus een **Alert thresholds**-checklist (standaard 80% en 100%). Zie [Budgets Overview](#budgets-overview) en [Budget Alerts](#budget-alerts).

### Personality

De **Initial prompt** is de systeemprompt die de toon, rol en beslissingsregels definieert. Platte tekst, geen template-syntax. Zie [Personality and the Initial Prompt](#personality-prompt).

### Context

Het Context-veldengroep bevat drie selectievakjes, een tekstveld voor richtlijnen en de scope-invoervelden:

- Neem de bovenliggende reactie en eerdere antwoorden in dezelfde thread op.
- Neem de trust factor, accountleeftijd, ban-geschiedenis en recente reacties van de reactiemaker op.
- Neem de paginatitel, subtitel, beschrijving en meta-tags op.
- Een optioneel **Community guidelines**-tekstblok dat aan elke prompt wordt toegevoegd.
- **Restrict to specific pages** - URL-patroon-allowlist (één per regel). Leeg betekent tenant-breed.
- **Restrict to specific locales** - locale-allowlist via een dual-list picker. Leeg betekent elke locale.

Meer context levert betere beslissingen op, maar verhoogt de tokenkosten per run. Zie [Context Options](#context-options), [Community Guidelines](#community-guidelines) en [Scope: URL and Locale Filters](#scope-url-locale).

### Triggers

Kies minimaal één gebeurtenis uit de lijst. Voor vote-threshold- en flag-threshold-triggers moet je ook de drempelwaarde instellen. Het optionele veld **Delay before running** stelt de uitvoering uit nadat een trigger is geactiveerd (nuttig bij flag-drempels waar stemmen nog aan het bezinken zijn). Zie [Trigger Events Overview](#triggers-overview) en [Deferred Triggers](#trigger-deferred-delay).

### Allowed tool calls

Vink **Allow any tool calls** aan om de volledige toolset beschikbaar te stellen. Vink anders de specifieke tools aan die de agent mag gebruiken - niet-toegestane tools worden uit de toolset van het model verwijderd en geweigerd op het moment van aanroepen. De subsectie **Ban options** vergrendelt de destructieve ban-varianten (delete-all-comments, ban-by-IP) achter expliciete opt-ins. Zie [Allowed Tool Calls Overview](#tools-overview) en [Tool: ban_user](#tool-ban-user).

### Approvals

Vink de acties aan die door een mens moeten worden goedgekeurd voordat de agent ze uitvoert. Approvals gelden alleen voor tools die de agent mag aanroepen; niet-toegestane tools worden direct geweigerd. In de EU-regio is **ban_user** vergrendeld ingeschakeld op grond van Artikel 17 van de Digital Services Act. Zie [Approval Workflow](#approval-workflow) en [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Approval notifications

Als approvals zijn ingeschakeld, kies je wie er een e-mail krijgt:

- **All admins and moderators** - accounteigenaren, super admins en comment moderator-admins.
- **Specific users** - handmatig gekozen via een dual-list picker.

De individuele bezorgfrequentie van elke reviewer (direct, uurlijks digest, dagelijks digest) wordt ingesteld in hun eigen profiel. Zie [Approval Notifications](#approval-notifications).

### Stats

Alleen-lezen. Totaal aantal runs, tijdstempel van de laatste run en het ID van de meest recente reactie die de agent heeft geschreven (indien aanwezig).

### Save

Klik op **Save agent**. De pagina verwijst terug naar de agentenlijst. Nieuwe agents zijn direct in aanmerking om triggers te ontvangen in dry-run.

### Later bewerken

Elke rij op de agentenlijstpagina biedt per-agent acties: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals** en **Delete**. Het bewerken van een agent heeft geen terugwerkende kracht op al geregistreerde runs - de geschiedenis blijft behouden. Replay-snapshots bevriezen ook de configuratie van de agent op het moment dat de replay is gestart, zodat de resultaten van een opgeslagen replay reproduceerbaar blijven, zelfs nadat je de prompt hebt bewerkt.