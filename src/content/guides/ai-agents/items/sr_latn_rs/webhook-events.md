Postoje četiri tipa webhook događaja agenta. Svaki događaj ima numeričku enum vrednost (koja se koristi u payload-ima) i kanonično tekstualno ime (koje se koristi u polju `event` u envelope-u i u HTTP header-u `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | An agent run completes with status `SUCCESS`. |
| `trigger.failed` | 1 | An agent run completes with status `ERROR`. |
| `approval.requested` | 2 | An approval is queued in `PENDING` state. |
| `approval.decided` | 3 | An approval transitions to `APPROVED`, `REJECTED`, or `EXECUTION_FAILED`. |

### `trigger.succeeded`

Okida nakon što se izvršavanje agenta završi bez greške. Polje `data` u payload-u sadrži:

- `triggerId` - jedinstveni ID izvršavanja.
- `triggerType` - [trigger reason enum](#triggers-overview) koji je pokrenuo izvršavanje.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokeni potrošeni u ovom izvršavanju.
- `wasDryRun` - true ako je agent bio u [dry-run mode](#dry-run-mode).
- `actions` - niz zapisa `TenantAgentAction` (pogledajte [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - ako ih je okidač imao.

Ako izvršavanje nije preduzelo nijednu akciju, niz `actions` je prazan - ovo je uspešno izvršavanje u kojem je agent odlučio da ne uradi ništa, što je korisno znati.

### `trigger.failed`

Okida kada izvršavanje prijavi grešku. Isti oblik payload-a kao kod `trigger.succeeded`, sa `status: 'ERROR'` i dodatnim poljem `errorMessage` koje opisuje šta je pošlo po zlu. Moguće greške uključuju neuspehe poziva LLM-a, neuspehe pri pozivanju alata i iscrpljivanje budžeta tokom izvršavanja.

Niz `actions` i dalje može sadržati unose za pozive alata koji su se završili pre greške.

### `approval.requested`

Okida u trenutku kada se odobrenje stavi u stanje `PENDING`. Payload sadrži:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - argumenti alata **propušteni doslovno** iz LLM poziva. Struktura zavisi od alata i nije stabilan javni ugovor - šema se može promeniti kako se budu dodavali novi alati.
- `createdAt`.
- `justification`, `confidence` - ako ih je agent obezbedio.
- `contextSnapshot` - kontekst komentara/stranice na koji se odobrenje odnosi.

Korisno za prosleđivanje čekajućih odobrenja u chat ops kanal: Slack bot pretplaćen na `approval.requested` može objaviti akciju i obrazloženje u kanal za moderaciju radi brzog pregleda.

### `approval.decided`

Okida kada odobrenje izađe iz stanja `PENDING`. Payload sadrži:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, ili `EXECUTION_FAILED`.
- `decidedBy` - ID korisnika moderatora koji je doneo odluku.
- `decidedAt` - kada je odluka doneta.
- `executedAt` - ako je APPROVED, kada je platforma izvršila odobrenu akciju.
- `executionResult` - ako je APPROVED, string koji opisuje rezultat izvršitelja.
- `contextSnapshot` - kontekst komentara/stranice.

Ovaj događaj pokriva sve ishode odluke:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` je postavljen, `executionResult` je poruka o uspehu.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` je postavljen, `executionResult` opisuje neuspeh.
- **Rejected** -> `status: REJECTED`, `executedAt` je null, `executionResult` je null.

### Header

Svaka isporuka uključuje HTTP header `X-FastComments-Agent-Event` sa kanoničnim tekstualnim imenom događaja (`trigger.succeeded`, itd.). Korisno ako je vaša krajnja tačka jedna URL koja obrađuje više tipova događaja.

### See also

- [Webhook Payloads](#webhook-payloads) for full per-event payload schemas.
- [Webhook Signing](#webhook-signing) for the HMAC scheme.
- [Webhook Retries](#webhook-retries) for delivery semantics.