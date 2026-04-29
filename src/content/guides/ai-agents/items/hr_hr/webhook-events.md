Postoje četiri tipa događaja (eventa) webhooka agenta. Svaki događaj ima numeričku vrijednost u enumeraciji (koristi se u payloadima) i kanonsko tekstualno ime (koristi se u polju omotnice `event` i u HTTP zaglavlju `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | Izvršavanje agenta se završava sa statusom `SUCCESS`. |
| `trigger.failed` | 1 | Izvršavanje agenta se završava sa statusom `ERROR`. |
| `approval.requested` | 2 | Odobrenje je stavljeno u red u stanju `PENDING`. |
| `approval.decided` | 3 | Odobrenje prelazi u `APPROVED`, `REJECTED`, ili `EXECUTION_FAILED`. |

### `trigger.succeeded`

Okida nakon što se izvršavanje agenta završi bez greške. Polje `data` u payloadu uključuje:

- `triggerId` - jedinstveni ID izvršavanja.
- `triggerType` - [trigger reason enum](#triggers-overview) koji je pokrenuo izvršavanje.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokeni potrošeni u ovom izvršavanju.
- `wasDryRun` - true ako je agent bio u [način suhog pokretanja](#dry-run-mode).
- `actions` - niz zapisa `TenantAgentAction` (vidi [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - ako je trigger imao te vrijednosti.

Ako je izvršavanje obavilo nula akcija, niz `actions` je prazan - radi se o uspješnom izvršavanju u kojem je "agent odlučio ne učiniti ništa", što je korisna informacija.

### `trigger.failed`

Okida kada izvršavanje uzrokuje grešku. Isti oblik payloada kao i za `trigger.succeeded`, sa `status: 'ERROR'` i dodatnim poljem `errorMessage` koje opisuje što je pošlo po zlu. Moguće pogreške uključuju neuspjehe poziva LLM-a, neuspjehe pri dispatchanju alata i iscrpljenje budžeta usred izvršavanja.

`actions` i dalje može sadržavati unose za pozive alata koji su završili prije greške.

### `approval.requested`

Okida u trenutku kada je odobrenje stavljeno u red u stanju `PENDING`. Payload uključuje:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - argumenti alata **proslijeđeni doslovno** iz poziva LLM-a. Oblik je specifičan za alat i nije stabilan javni ugovor - shema se može mijenjati kako se dodaju novi alati.
- `createdAt`.
- `justification`, `confidence` - ako ih je agent dao.
- `contextSnapshot` - kontekst komentara/stranice na koji se odobrenje odnosi.

Koristan za prosljeđivanje na čekanju odobrenja u kanal za chat-ops: Slack bot pretplaćen na `approval.requested` može objaviti akciju i obrazloženje u kanalu za moderaciju radi brzog pregleda.

### `approval.decided`

Okida kada odobrenje izađe iz stanja `PENDING`. Payload uključuje:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, ili `EXECUTION_FAILED`.
- `decidedBy` - ID korisnika moderatora koji je donio odluku.
- `decidedAt` - kada je donio odluku.
- `executedAt` - ako je APPROVED, kada je platforma izvršila odobrenu akciju.
- `executionResult` - ako je APPROVED, string koji opisuje rezultat izvršitelja.
- `contextSnapshot` - kontekst komentara/stranice.

Ovaj događaj pokriva sve ishode odluke:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` postavljeno, `executionResult` je poruka o uspjehu.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` postavljeno, `executionResult` opisuje neuspjeh.
- **Rejected** -> `status: REJECTED`, `executedAt` je null, `executionResult` je null.

### Header

Svaka isporuka uključuje HTTP zaglavlje `X-FastComments-Agent-Event` s kanonskim tekstualnim imenom događaja (`trigger.succeeded`, itd.). Korisno ako je vaš endpoint jedna URL adresa koja obrađuje više tipova događaja.

### See also

- [Webhook Payloads](#webhook-payloads) za potpune sheme payloada po događaju.
- [Webhook Signing](#webhook-signing) za HMAC shemu.
- [Webhook Retries](#webhook-retries) za semantiku isporuke.