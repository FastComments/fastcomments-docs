Postoje četiri vrste agent webhook događaja. Svaki događaj ima numeričku enum vrijednost (koristi se u payloadima) i kanonski string naziv (koristi se u `event` envelope polju i u `X-FastComments-Agent-Event` HTTP zaglavlju).

| Naziv događaja | Enum | Aktivira se kada |
|---|---|---|
| `trigger.succeeded` | 0 | Agentov run završi sa statusom `SUCCESS`. |
| `trigger.failed` | 1 | Agentov run završi sa statusom `ERROR`. |
| `approval.requested` | 2 | Odobrenje se stavi u red u `PENDING` stanju. |
| `approval.decided` | 3 | Odobrenje prijeđe u `APPROVED`, `REJECTED` ili `EXECUTION_FAILED`. |

### `trigger.succeeded`

Aktivira se nakon što agentov run završi bez greške. Payloadovo `data` polje uključuje:

- `triggerId` - jedinstveni ID runa.
- `triggerType` - [enum razloga okidanja](#triggers-overview) koji je pokrenuo run.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokeni potrošeni u ovom runu.
- `wasDryRun` - true ako je agent bio u [dry-run načinu rada](#dry-run-mode).
- `actions` - niz `TenantAgentAction` zapisa (vidi [Webhook Payloadi](#webhook-payloads)).
- `commentId`, `url`, `urlId` - ako ih je okidač imao.

Ako run nije poduzeo nijednu akciju, `actions` niz je prazan - to je uspješan run "agent je odlučio ne poduzeti ništa", što je korisno znati.

### `trigger.failed`

Aktivira se kada run doživi grešku. Isti oblik payloada kao `trigger.succeeded`, sa `status: 'ERROR'` i dodatnim `errorMessage` poljem koje opisuje što je pošlo naopako. Moguće greške uključuju neuspjehe LLM poziva, neuspjehe slanja alata i iscrpljenost budžeta usred runa.

`actions` i dalje može sadržavati unose za pozive alata koji su završili prije greške.

### `approval.requested`

Aktivira se u trenutku kada se odobrenje stavi u red u `PENDING` stanju. Payload uključuje:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - argumenti alata **proslijeđeni doslovno** iz LLM poziva. Oblik je specifičan po alatu i nije stabilan javni ugovor - shema se može mijenjati kako se dodaju novi alati.
- `createdAt`.
- `justification`, `confidence` - ako ih je agent dostavio.
- `contextSnapshot` - kontekst komentara / stranice na koji se odobrenje odnosi.

Korisno za prosljeđivanje odobrenja koja čekaju u chat ops kanal: Slack bot pretplaćen na `approval.requested` može objaviti akciju i obrazloženje u moderacijski kanal za pregled na prvi pogled.

### `approval.decided`

Aktivira se kada odobrenje izađe iz `PENDING` stanja. Payload uključuje:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` ili `EXECUTION_FAILED`.
- `decidedBy` - korisnički ID moderatora koji je odlučio.
- `decidedAt` - kada je odlučio.
- `executedAt` - ako je APPROVED, kada je platforma izvršila odobrenu akciju.
- `executionResult` - ako je APPROVED, string koji opisuje rezultat izvršitelja.
- `contextSnapshot` - kontekst komentara / stranice.

Ovaj događaj pokriva sve ishode odluke:

- **Odobreno + uspješno izvršeno** -> `status: APPROVED`, `executedAt` postavljeno, `executionResult` je poruka uspjeha.
- **Odobreno + izvršitelj nije uspio** -> `status: EXECUTION_FAILED`, `executedAt` postavljeno, `executionResult` opisuje neuspjeh.
- **Odbijeno** -> `status: REJECTED`, `executedAt` je null, `executionResult` je null.

### Zaglavlje

Svaka dostava uključuje `X-FastComments-Agent-Event` HTTP zaglavlje s kanonskim string nazivom događaja (`trigger.succeeded`, itd.). Korisno ako je vaša krajnja točka jedan URL koji obrađuje više vrsta događaja.

### Vidi također

- [Webhook Payloadi](#webhook-payloads) za potpune sheme payloada po događaju.
- [Potpisivanje Webhooka](#webhook-signing) za HMAC shemu.
- [Ponovni pokušaji Webhooka](#webhook-retries) za semantiku dostave.