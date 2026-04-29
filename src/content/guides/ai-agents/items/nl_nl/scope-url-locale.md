Standaard draait een agent over je hele tenant - elke pagina, elke locale. De **Scope** en **Locales** secties op het bewerkingsformulier laten je dat beperken.

### Beperk tot specifieke pagina's

Het veld **Restrict to specific pages** accepteert één URL-patroon per regel, in url-pattern glob syntax. De agent draait alleen op reacties waarvan de pagina-URL overeenkomt met ten minste één van de patronen. Voorbeelden:

- `/news/*` - elke pagina onder `/news`.
- `/forums/*` - elke pagina onder `/forums`.
- `/blog/2026/*` - elke pagina onder `/blog/2026`.
- (meerdere regels samen) - de agent draait als **een** van de patronen overeenkomt.

Maximum: 50 patronen per agent. Patronen moeten geldige url-pattern globs zijn - het formulier weigert foutieve patronen met een specifieke foutmelding.

Wanneer het veld **leeg** is, draait de agent op elke pagina in de tenant.

Wanneer het veld **niet-leeg** is, werkt de agent volgens het fail-closed-principe: elke trigger waarvan de reactie geen `urlId` heeft (bijv. tenant-niveau gebeurtenissen zonder pagina-context) wordt overgeslagen. Dit is opzettelijk - "gescopeerd naar /news/*" mag niet stilletjes vallen terug naar "alles".

### Beperk tot specifieke locales

De dual-list picker **Restrict to specific locales** accepteert FastComments locale-ID's (`en_us`, `zh_cn`, `de_de`, etc.). De agent draait alleen op reacties waarvan de gedetecteerde locale in de geselecteerde lijst staat.

De gedetecteerde locale komt uit het `locale`-veld van de reactie, dat door de comment-widget bij het plaatsen wordt ingesteld op basis van de paginalocale.

Wanneer **geen locales** zijn geselecteerd, draait de agent op elke locale.

Wanneer **één of meer locales** zijn geselecteerd, werkt de agent volgens het fail-closed-principe: triggers zonder reactie, of triggers op reacties zonder `locale`-veld, worden overgeslagen.

### Gecombineerde afbakening

URL- en locale-filters worden met EN gecombineerd. Een trigger activeert de agent alleen als **beide** filters het toestaan.

Nuttige combinaties:
- `/news/*` URL-patroon + `en_us` locale - alleen de Engelse nieuwsrubriek.
- Geen URL-filter + meerdere locales - tenant-breed, maar alleen voor de talen waarvoor de prompt van deze agent geschreven is.

### Waarom een agent afbakenen

- **Kosten.** Afbakening vermindert het aantal triggers dat de agent moet verwerken, en verlaagt daarmee het tokenverbruik.
- **Correctheid.** Een samenvattingsprompt die is afgestemd op technische artikelen kan slechte output geven op productpagina's. Afbakening is een scherper instrument dan de prompt vragen "non-technische pagina's over te slaan" in het Engels.
- **Locale-specifiek gedrag.** Een welkomstgroet die alleen in het Duits schrijft, zou alleen op reacties met Duitse locale moeten draaien. Combineer `de_de` locale-afbakening met een Duitstalige toon in de [initiële prompt](#personality-prompt).

### Wat afbakening niet doet

- Het verandert niet het **aantal agent-slots** (zie [Plans and Eligibility](#plans-and-eligibility)) - een afgebakende agent neemt nog steeds één slot in beslag.
- Het verandert niet de [Budget caps](#budgets-overview) - de dagelijkse en maandelijkse limieten per agent gelden voor alle overeenkomende triggers.
- Het schaalt niet achteraf eerdere runs in - de uitvoeringsgeschiedenis toont alles wat de agent heeft gedaan, zelfs als je de afbakening later aanscherpt.