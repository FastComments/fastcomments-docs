AI Agents zijn beschikbaar op de **Flex** en **Pro** plannen. Het **Creator**-plan bevat ze niet.

### Limieten op planniveau

Elke plantier stelt:

- **Standaard dag- en maandelijkse budgetlimieten.** U kunt deze per agent verlagen; het verhogen van de limiet per account vereist een plan met een hoger plafond. Zie [Budgetoverzicht](#budgets-overview).

De exacte cijfers worden weergegeven op de [prijspagina](https://fastcomments.com/traffic-pricing) en op de factureringspagina van uw account. Ze worden ook inline weergegeven op het agentbewerkingsformulier, zodat u het formulier nooit hoeft te verlaten om uw limiet te vinden.

FastComments Pro bevat $200/maand aan AI-gebruik. Flex wordt gefactureerd tegen een tarief van $20 per miljoen tokens voor alle modellen (momenteel GLM 5.1 of gpt-oss-120B-turbo).

### Facturering moet geldig zijn

AI Agents draaien alleen wanneer de tenant **geldige factureringsgegevens** heeft. Als de betaalmethode ongeldig wordt, worden alle agents gepauzeerd en toont de AI Agents-pagina een banner die degene met de rol **Billing Admin** verzoekt de facturering bij te werken. Agents hervatten automatisch zodra de facturering is hersteld — er vindt geen replay of backfill plaats van triggers die tijdens de storing werden geactiveerd.

Dit is een harde voorwaarde: tokenuitgaven worden op uw account gefactureerd, dus het platform zal geen enkele LLM-aanroep uitvoeren zonder een werkende betaalmethode.

### Wie agents kan beheren

De agentbeheerpagina's zijn afgeschermd achter de dashboardrol **Customization Admin**. **Comment Moderator Admins** kunnen goedkeuringen beoordelen en beslissen (zie [Goedkeuringsworkflow](#approval-workflow)), maar kunnen geen agents aanmaken of bewerken. **Billing Admins** ontvangen [e-mails met budgetwaarschuwingen](#budget-alerts) ongeacht of ze toegang tot agents hebben.

---