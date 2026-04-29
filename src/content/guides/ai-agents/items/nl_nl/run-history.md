Rungeschiedenis is het per-agent logboek van elke trigger die is uitgevoerd. Toegankelijk vanaf de agentenlijst via de **Uitvoeringen**-knop, of rechtstreeks op `/auth/my-account/ai-agents/{agentId}/runs`.

### Wat staat er op de pagina

Een gepagineerde tabel met één rij per run:

| Kolom | Betekenis |
|---|---|
| Datum | Wanneer de trigger afging (of wanneer de uitgestelde trigger draaide). |
| Status | **Gestart**, **Succes**, of **Fout**. Er wordt een **Dry Run**-badge weergegeven als de run in dry-run-modus was. |
| Kosten | Kosten per run in de valuta van je tenant. Leeg bij lopende (Gestart) runs. |
| Acties | Het aantal tool-aanroepen in de run. |
| Details | Een **Bekijken**-knop die de [Run detailweergave](#run-detail-view) opent. |

### Betekenis van statussen

- **Gestart** - de run is in uitvoering, of is gestopt voordat deze voltooid was. Een run die ongewoon lang in **Gestart** blijft, wijst meestal op een LLM-aanroep-timeout.
- **Fout** - de run is voltooid maar is ergens mislukt - een LLM-aanroep retourneerde een fout, het dispatchen naar een tool mislukte, enz. De detailweergave bevat de specifieke fout.
- **Succes** - de run is voltooid zonder fouten. De agent kan nul, één of meerdere acties hebben ondernomen.

### Lege staat

Wanneer een agent geen runs heeft, toont de pagina: "Nog geen runs voor deze agent. Ingeschakelde runs verschijnen hier zodra een trigger afgaat; gebruik Test run om te bekijken wat deze agent zou doen met eerdere opmerkingen."

Dat laatste is opzettelijk - de [test run flow](#test-runs-replays) is de aanbevolen manier om Rungeschiedenis te vullen voor een nieuwe agent.

### Wat staat er niet op de rungeschiedenispagina

- **Live-triggers die nooit dispatched zijn** - een trigger die werd geblokkeerd vanwege budget, scope of rate limiting verschijnt niet op deze pagina. Deze verschijnen op de [Analytics-pagina](#analytics-page) onder "Triggers overgeslagen".
- **Goedkeuringen** - openstaande goedkeuringen voor acties die in deze run zijn genomen, staan in de [approvals inbox](#approval-workflow). De actie verschijnt in de run-detailweergave als **In afwachting van goedkeuring**.

### Bewaartermijn

Individuele runrecords worden 90 dagen bewaard, waarna de run uit de geschiedenis verdwijnt. Kosten- en triggeraantallen blijven echter worden samengevat in langetermijn-analyses, dus de [Analytics-pagina](#analytics-page) toont nog steeds historische totalen buiten dat venster.

### Replays

Op replays gebaseerde runs zijn standaard uitgesloten van de live-runs-weergave. De pagina [Test Runs (Replays)](#test-runs-replays) is waar je die kunt zien.

### Filteren over meerdere agenten

De runstabel is per agent. Er is geen cross-agent runs-weergave - de [Analytics-pagina](#analytics-page) is de cross-agent samenvatting. Als je runs over meerdere agenten moet inspecteren, zijn de [Webhooks](#webhooks-overview) `trigger.succeeded` en `trigger.failed` events wat je naar je eigen systeem zou doorsturen.