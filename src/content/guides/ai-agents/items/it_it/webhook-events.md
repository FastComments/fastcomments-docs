Ci sono quattro tipi di eventi webhook dell'agente. Ogni evento ha un valore enum numerico (usato nei payload) e un nome stringa canonico (usato nel campo `event` dell'envelope e nell'intestazione HTTP `X-FastComments-Agent-Event`).

| Nome evento | Enum | Si attiva quando |
|---|---|---|
| `trigger.succeeded` | 0 | Una esecuzione dell'agente termina con stato `SUCCESS`. |
| `trigger.failed` | 1 | Una esecuzione dell'agente termina con stato `ERROR`. |
| `approval.requested` | 2 | Viene accodata una approvazione nello stato `PENDING`. |
| `approval.decided` | 3 | Un'approvazione transita in `APPROVED`, `REJECTED` o `EXECUTION_FAILED`. |

### `trigger.succeeded`

Si attiva dopo che l'esecuzione dell'agente termina senza errori. Il campo `data` del payload include:

- `triggerId` - l'ID univoco dell'esecuzione.
- `triggerType` - l'[enum dei motivi del trigger](#triggers-overview) che ha avviato l'esecuzione.
- `status` - `SUCCESS` (stringa).
- `tokensUsed` - token consumati in questa esecuzione.
- `wasDryRun` - true se l'agente era in [modalità dry-run](#dry-run-mode).
- `actions` - array di record `TenantAgentAction` (vedi [Payload dei webhook](#webhook-payloads)).
- `commentId`, `url`, `urlId` - se il trigger li aveva.

Se l'esecuzione ha compiuto zero azioni, l'array `actions` è vuoto - si tratta di una esecuzione riuscita in cui "l'agente ha deciso di non fare nulla", informazione utile da conoscere.

### `trigger.failed`

Si attiva quando un'esecuzione genera un errore. Stesso formato del payload di `trigger.succeeded`, con `status: 'ERROR'` e un campo aggiuntivo `errorMessage` che descrive cosa è andato storto. Errori possibili includono fallimenti di chiamate LLM, fallimenti di dispatch degli strumenti, e l'esaurimento del budget durante l'esecuzione.

`actions` può comunque contenere voci per le chiamate agli strumenti completate prima dell'errore.

### `approval.requested`

Si attiva nel momento in cui un'approvazione viene accodata nello stato `PENDING`. Il payload include:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - gli argomenti dello strumento **trasmessi integralmente** dalla chiamata LLM. La forma dipende dallo strumento e non è un contratto pubblico stabile - lo schema può cambiare con l'aggiunta di nuovi strumenti.
- `createdAt`.
- `justification`, `confidence` - se l'agente li ha forniti.
- `contextSnapshot` - il contesto del commento / della pagina a cui l'approvazione si riferisce.

Utile per inoltrare le approvazioni in sospeso in un canale di chat ops: un bot Slack iscritto a `approval.requested` può pubblicare l'azione e le motivazioni in un canale di moderazione per una revisione a colpo d'occhio.

### `approval.decided`

Si attiva quando un'approvazione esce da `PENDING`. Il payload include:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, o `EXECUTION_FAILED`.
- `decidedBy` - l'ID utente del moderatore che ha preso la decisione.
- `decidedAt` - quando ha deciso.
- `executedAt` - se APPROVED, quando la piattaforma ha eseguito l'azione approvata.
- `executionResult` - se APPROVED, una stringa che descrive il risultato dell'esecutore.
- `contextSnapshot` - il contesto del commento / della pagina.

Questo evento copre tutti gli esiti della decisione:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` impostato, `executionResult` è il messaggio di successo.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` impostato, `executionResult` descrive il fallimento.
- **Rejected** -> `status: REJECTED`, `executedAt` è null, `executionResult` è null.

### Intestazione

Ogni consegna include un'intestazione HTTP `X-FastComments-Agent-Event` con il nome stringa canonico dell'evento (`trigger.succeeded`, ecc.). Utile se il tuo endpoint è un'unica URL che gestisce più tipi di eventi.

### Vedi anche

- [Payload dei webhook](#webhook-payloads) per gli schemi completi dei payload per evento.
- [Firma dei webhook](#webhook-signing) per lo schema HMAC.
- [Ritenti dei webhook](#webhook-retries) per la semantica di consegna.