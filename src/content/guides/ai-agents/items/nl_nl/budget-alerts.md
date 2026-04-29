Budgetwaarschuwing-e-mails worden verzonden wanneer het verbruik van een agent een configureerbaar percentage van zijn limiet overschrijdt. Ze gaan naar de personen die de factuur beheren.

### Hoe waarschuwingen werken

Elke agent heeft een veld **Alert thresholds** op het bewerkformulier. Standaard is dit `80%` en `100%`. Je kunt individuele drempels aan- of uitvinken en andere percentages toevoegen.

Wanneer het verbruik van de agent binnen een bepaalde reikwijdte (dagelijks of maandelijks) voor de eerste keer in die periode een drempel overschrijdt, stuurt het platform één e-mail per ontvanger. Het opnieuw overschrijden van de drempel later in dezelfde periode (bijv. verbruik daalde onder 80% en ging weer boven) stuurt **niet** opnieuw.

Dit geldt per periode: een nieuwe dagelijkse reset start de drempel-overschrijdingslogica opnieuw voor die dag.

### Waarschuwingen op tenantniveau

De tenant (account) heeft eigen dagelijkse en maandelijkse limieten. Waarschuwingen op tenantniveau worden geactiveerd bij vaste drempels (`80%` en `100%`). Deze zijn niet per agent configureerbaar omdat ze voor de hele tenant gelden.

### Ontvangers

Budgetwaarschuwingen worden verzonden naar:

- Elke gebruiker die is gemarkeerd als **Superbeheerder** op de tenant.
- Elke gebruiker die is gemarkeerd als **Facturatiebeheerder** op de tenant.

Dat omvat de unie van beide rollen - een gebruiker met beide rollen ontvangt één e-mail.

### Waarom beide rollen

Superbeheerders zijn doorgaans de operators die moeten weten dat een agent zijn limiet bereikt. Facturatiebeheerders zijn eigenaar van de factuur en moeten op de hoogte zijn van kostenpieken, ongeacht of ze agenten dagelijks beheren. Om de agent daadwerkelijk te bewerken (het plafond verhogen, pauzeren), heeft de ontvanger ook de rol **Aanpassingsbeheerder** nodig - die toegang regelt tot de agent-bewerkpagina.

### Per-gebruiker uitschrijving

Ontvangers die zich op hun profiel hebben afgemeld voor beheerdersmeldingen worden overgeslagen. Dit is dezelfde uitschakeloptie die andere beheerdersmeldingen regelt.

Als **alle** ontvangers zijn afgemeld, wordt de waarschuwing gelogd (waarschuwingsniveau) en wordt er geen e-mail verzonden.

### E-mailinhoud

De e-mail bevat:

- De **weergavenaam van de agent** en interne naam.
- De **reikwijdte** die is overschreden (bijv. "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- Het **drempelpercentage** dat is overschreden.
- **Verbruik** in de valuta van de tenant.
- **Plafond** in de valuta van de tenant.
- Een **één-klik ondertekende inloglink** die de ontvanger direct brengt naar:
  - De agent-bewerkpagina, voor waarschuwingen op agentniveau.
  - De AI Agents-lijstpagina, voor waarschuwingen op tenantniveau.

De link is vooraf geauthenticeerd, zodat de ontvanger met één klik het plafond kan verhogen of de agent kan uitschakelen.

### Hoe drempels afgaan

Het platform houdt bij welke drempels al zijn afgevuurd deze periode, afzonderlijk voor de agent en de tenant. Dus:

- Het overschrijden van 80% en daarna 100% in dezelfde periode activeert beide, op volgorde.
- Rechtstreeks van 0% naar 100% in één grote sprong activeert de **hoogste** overschreden drempel (100%), niet 80%, zodat de meest ernstige waarschuwing wordt verzonden.

### Wanneer je geen meldingen meer krijgt

Als het verbruik van de agent deze periode nooit de volgende drempel bereikt, ontvang je die periode geen verdere e-mails. De volgende dagelijkse reset (of maandelijkse reset) wist de tracking.

### Waarschuwingen uitschakelen

Vink de drempel uit die je niet wilt. Als je geen meldingen op een specifieke agent wilt, vink dan alle percentages uit. Waarschuwingen op tenantniveau kunnen niet per agent worden uitgeschakeld (ze gelden voor de hele tenant).

### Zie ook

- [Budgetoverzicht](#budgets-overview).
- [Redenen voor uitsluiting](#drop-reasons) - wat er gebeurt wanneer het plafond volledig is bereikt.
- [Kostmodel](#cost-model) - wat er wordt gemeten.