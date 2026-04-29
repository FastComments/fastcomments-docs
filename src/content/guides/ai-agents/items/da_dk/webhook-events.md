Der er fire agent webhook-begivenhedstyper. Hver begivenhed har en numerisk enum-værdi (bruges i payloads) og et kanonisk strengnavn (bruges i `event`-feltet i kuverten og i `X-FastComments-Agent-Event` HTTP-headeren).

| Event name | Enum | Udløses når |
|---|---|---|
| `trigger.succeeded` | 0 | En agentkørsel afsluttes med status `SUCCESS`. |
| `trigger.failed` | 1 | En agentkørsel afsluttes med status `ERROR`. |
| `approval.requested` | 2 | En godkendelse sættes i kø i `PENDING`-tilstand. |
| `approval.decided` | 3 | En godkendelse skifter til `APPROVED`, `REJECTED` eller `EXECUTION_FAILED`. |

### `trigger.succeeded`

Udløses efter agentens kørsel afslutter uden fejl. Payloadens `data`-felt indeholder:

- `triggerId` - den unikke kørsels-id.
- `triggerType` - [trigger reason enum](#triggers-overview), der startede kørslen.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokens forbrugt i denne kørsel.
- `wasDryRun` - true hvis agenten var i [dry-run-tilstand](#dry-run-mode).
- `actions` - array af `TenantAgentAction`-poster (se [Webhook-payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - hvis triggeren havde dem.

Hvis kørslen ikke udførte nogen handlinger, er `actions`-arrayet tomt - dette er en succesfuld "agenten besluttede ikke at gøre noget"-kørsel, hvilket er nyttigt at vide.

### `trigger.failed`

Udløses når en kørsel fejler. Samme payload-form som `trigger.succeeded`, med `status: 'ERROR'` og et ekstra `errorMessage`-felt, der beskriver hvad der gik galt. Mulige fejl inkluderer LLM-opkaldsfejl, fejl ved værktøjs-dispatch og budgetudtømning midt i kørslen.

`actions` kan stadig indeholde poster for værktøjsopkald, der blev fuldført før fejlen.

### `approval.requested`

Udløses i det øjeblik en godkendelse sættes i kø i `PENDING`-tilstand. Payloaden indeholder:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - værktøjets argumenter **videregivet ordret** fra LLM-opkaldet. Strukturen er per-værktøj og ikke et stabilt offentligt kontrakt - skemaet kan ændre sig, når nye værktøjer tilføjes.
- `createdAt`.
- `justification`, `confidence` - hvis agenten leverede dem.
- `contextSnapshot` - kommentar-/sidekonteksten, som godkendelsen relaterer til.

Nyttigt til at videresende ventende godkendelser til en chat-ops-kanal: en Slack-bot, der er tilmeldt `approval.requested`, kan poste handlingen og begrundelsen i en moderationskanal for hurtig gennemgang.

### `approval.decided`

Udløses når en godkendelse går ud af `PENDING`. Payloaden indeholder:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, eller `EXECUTION_FAILED`.
- `decidedBy` - bruger-id'et for den moderator, der tog beslutningen.
- `decidedAt` - hvornår de besluttede.
- `executedAt` - hvis APPROVED, hvornår platformen udførte den godkendte handling.
- `executionResult` - hvis APPROVED, en streng, der beskriver eksekutorens resultat.
- `contextSnapshot` - kommentar-/sidekonteksten.

Denne begivenhed dækker alle beslutningsresultater:

- **Godkendt + eksekveret uden problemer** -> `status: APPROVED`, `executedAt` sat, `executionResult` er succesbeskeden.
- **Godkendt + eksekutor fejlede** -> `status: EXECUTION_FAILED`, `executedAt` sat, `executionResult` beskriver fejlen.
- **Afvist** -> `status: REJECTED`, `executedAt` er null, `executionResult` er null.

### Header

Hver levering inkluderer en `X-FastComments-Agent-Event` HTTP-header med begivenhedens kanoniske strengnavn (`trigger.succeeded` osv.). Nyttigt hvis dit endpoint er en enkelt URL, der håndterer flere begivenhedstyper.

### Se også

- [Webhook-payloads](#webhook-payloads) for fulde skemaer pr. begivenhed.
- [Webhook-signering](#webhook-signing) for HMAC-skemaet.
- [Webhook-genforsøg](#webhook-retries) for leveringssemantik.