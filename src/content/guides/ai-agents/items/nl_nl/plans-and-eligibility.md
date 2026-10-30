AI Agents zijn beschikbaar op de **Flex**- en **Pro**-plannen. Het Creator‑plan omvat ze niet.

### Limieten per plan

Elke planlaag stelt het volgende in:

- **Standaard dagelijkse en maandelijkse budgetlimieten.** Je kunt deze per agent verlagen; het verhogen van de limiet per account vereist een plan met een hoger plafond. Zie [Budgets Overview](#budgets-overview).

De exacte cijfers worden weergegeven op de [prijspagina](https://fastcomments.com/traffic-pricing) en op de facturatiepagina van je account. Ze worden ook inline weergegeven op het bewerkingsformulier van de agent, zodat je nooit het formulier hoeft te verlaten om je limiet te vinden.

FastComments Pro omvat $200/maand aan AI‑gebruik. Flex wordt gefactureerd tegen een tarief van $20 per miljoen tokens voor alle modellen (momenteel ofwel GLM 5.2 of gpt-oss-120B).

### Facturatie moet geldig zijn

AI Agents draaien alleen wanneer de huurder **geldige facturatie in het bestand** heeft. Als de betaalmethode ongeldig wordt, worden alle agents gepauzeerd en toont de AI Agents-pagina een banner die degene met de **Billing Admin**‑rol aanwijst om de facturatie bij te werken. Agents hervatten automatisch zodra de facturatie is hersteld – er is geen herhaling of backfill van triggers die tijdens de storing zijn geactiveerd.

Dit is een harde voorwaarde: tokenuitgaven worden gefactureerd op je account, dus het platform zal geen LLM‑aanroep verzenden zonder een werkende betaalmethode.

### Wie agents kan beheren

De agent‑adminpagina's zijn achter de dashboardrol **Customization Admin** afgeschermd. **Comment Moderator Admins** kunnen beoordelingen uitvoeren en goedkeuringen beslissen (zie [Approval Workflow](#approval-workflow)), maar kunnen geen agents aanmaken of bewerken. **Billing Admins** ontvangen [budgetwaarschuwing e-mails](#budget-alerts) ongeacht of ze toegang tot agents hebben.