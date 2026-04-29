Er zijn vier agent-webhook-gebeurtenistypen. Elk evenement heeft een numerieke enum-waarde (gebruikt in payloads) en een canonieke stringnaam (gebruikt in het `event` envelope-veld en in de `X-FastComments-Agent-Event` HTTP-header).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | Wordt geactiveerd wanneer een agent-run eindigt met status `SUCCESS`. |
| `trigger.failed` | 1 | Wordt geactiveerd wanneer een agent-run eindigt met status `ERROR`. |
| `approval.requested` | 2 | Wordt geactiveerd wanneer een goedkeuring in staat `PENDING` in de wachtrij wordt geplaatst. |
| `approval.decided` | 3 | Wordt geactiveerd wanneer een goedkeuring overgaat naar `APPROVED`, `REJECTED`, of `EXECUTION_FAILED`. |

### `trigger.succeeded`

Wordt geactiveerd nadat de run van de agent zonder fouten is voltooid. Het `data`-veld van de payload bevat:

- `triggerId` - de unieke run-ID.
- `triggerType` - de [trigger reason enum](#triggers-overview) die de run heeft gestart.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokens verbruikt in deze run.
- `wasDryRun` - true als de agent in [dry-run mode](#dry-run-mode) was.
- `actions` - array van `TenantAgentAction`-records (zie [Webhook-payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - als de trigger die had.

Als de run nul acties uitvoerde, is de `actions`-array leeg - dit is een succesvolle "de agent besloot niets te doen"-run, wat nuttig kan zijn om te weten.

### `trigger.failed`

Wordt geactiveerd wanneer een run een fout produceert. Zelfde payload-structuur als `trigger.succeeded`, met `status: 'ERROR'` en een extra `errorMessage`-veld dat beschrijft wat er misging. Mogelijke fouten omvatten LLM-aanroepfouten, fouten bij het dispatchen van tools en het opraken van het budget halverwege de run.

De `actions`-array kan nog steeds vermeldingen bevatten voor tool-aanroepen die vóór de fout voltooid waren.

### `approval.requested`

Wordt geactiveerd op het moment dat een goedkeuring in de staat `PENDING` wordt geplaatst. De payload bevat:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - de argumenten van de tool **letterlijk doorgegeven** vanuit de LLM-aanroep. De vorm is per tool en geen stabiel openbaar contract - het schema kan veranderen wanneer nieuwe tools worden toegevoegd.
- `createdAt`.
- `justification`, `confidence` - als de agent deze heeft meegegeven.
- `contextSnapshot` - de commentaar-/pagina-context waarop de goedkeuring betrekking heeft.

Handig om in afwachting zijnde goedkeuringen door te sturen naar een chat-ops-kanaal: een Slack-bot die geabonneerd is op `approval.requested` kan de actie en de onderbouwing in een moderatiekanaal plaatsen voor een snelle beoordeling.

### `approval.decided`

Wordt geactiveerd wanneer een goedkeuring uit `PENDING` gaat. De payload bevat:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, of `EXECUTION_FAILED`.
- `decidedBy` - de gebruikers-ID van de moderator die de beslissing nam.
- `decidedAt` - wanneer de beslissing werd genomen.
- `executedAt` - als APPROVED, wanneer het platform de goedgekeurde actie heeft uitgevoerd.
- `executionResult` - als APPROVED, een string die het resultaat van de executor beschrijft.
- `contextSnapshot` - de commentaar-/pagina-context.

Dit event dekt alle besluituitkomsten:

- **Goedgekeurd + succesvol uitgevoerd** -> `status: APPROVED`, `executedAt` ingesteld, `executionResult` is het succesbericht.
- **Goedgekeurd + uitvoerder faalde** -> `status: EXECUTION_FAILED`, `executedAt` ingesteld, `executionResult` beschrijft de fout.
- **Afgewezen** -> `status: REJECTED`, `executedAt` is null, `executionResult` is null.

### Header

Elke levering bevat een `X-FastComments-Agent-Event` HTTP-header met de canonieke stringnaam van het event (`trigger.succeeded`, etc.). Handig als je endpoint één URL is die meerdere eventtypes afhandelt.

### Zie ook

- [Webhook-payloads](#webhook-payloads) voor volledige per-event payloadschema's.
- [Webhook-handtekening](#webhook-signing) voor het HMAC-schema.
- [Webhook-hertentingen](#webhook-retries) voor de bezorgingssemantiek.