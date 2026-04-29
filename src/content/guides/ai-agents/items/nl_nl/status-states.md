Een agent heeft één van de drie statussen:

### Uitgeschakeld

De agent is uitgeschakeld. Er worden geen triggers verwerkt en de agent verschijnt niet in het dispatchpad. De uitvoeringsgeschiedenis, analytics en het geheugen blijven behouden - als u hem later weer inschakelt, zijn de historische gegevens er nog.

Gebruik `Disabled` wanneer:
- U een agent uit de rotatie wilt halen zonder deze te verliezen.
- Een agent zich slecht gedraagt en u hem onmiddellijk moet stoppen terwijl u onderzoekt wat er aan de hand is.
- U agenten seizoensgebonden in- en uitzet (bijv. een alleen-met-vakantie begroeter).

### Dry Run — standaard voor nieuwe agenten

De agent draait end-to-end - hij verwerkt triggers, roept de LLM aan, kiest tool-aanroepen, berekent onderbouwingen en vertrouwen - maar **er worden geen echte acties uitgevoerd**. Elke run wordt vastgelegd met het **Dry Run**-badge in [Run History](#run-history).

Gebruik `Dry Run` wanneer:
- Een nieuwe agent net uit de doos is. Elk startertemplate komt in dry-run.
- U de prompt hebt bewerkt of de set triggers hebt aangepast en wilt zien hoe de wijziging uitpakt voordat u deze doorvoert.
- U een [testuitvoering / replay](#test-runs-replays) uitvoert (replays forceren dry-run ongeacht de status van de agent).

Het platform rekent tokens voor dry-run-uitvoeringen - de LLM-aanroep vindt nog steeds plaats, alleen de bijwerkingen worden overgeslagen. Budgetlimieten gelden ook voor dry-run. Zie [Budgets Overview](#budgets-overview).

### Ingeschakeld

De agent onderneemt echte acties. Tool-aanroepen worden uitgevoerd - of in de wachtrij geplaatst voor [approval](#approval-workflow) als de actie geblokkeerd is.

Gebruik `Enabled` nadat de output van dry-run er correct uitziet.

### Status wijzigen

U kunt op het bewerkingsformulier tussen twee statussen schakelen. Overschakelen van Dry Run naar Enabled voert de dry-run-acties niet retrospectief opnieuw uit - die blijven als dry-run-geschiedenis. Nieuwe triggers vanaf dat moment draaien live.

Overschakelen van Enabled naar Disabled halverwege een run beëindigt een lopende run **niet**. De momenteel uitgevoerde trigger wordt afgemaakt (met wat deze al is begonnen); de volgende trigger wordt gedropt omdat de agent nu Disabled is.

### Status bij factureringsproblemen

Als de facturering van uw tenant ongeldig wordt, worden alle agents effectief gepauzeerd ongeacht de opgeslagen status - triggers worden gedropt met `BILLING_INVALID` totdat de facturering is hersteld. Het opgeslagen statusveld wordt niet gewijzigd; de dispatcher weigert gewoon te draaien. Zie [Plans and Eligibility](#plans-and-eligibility).