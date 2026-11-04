Postoje četiri tipa agent webhook događaja. Svaki događaj ima numeričku enum vrednost (koristi se u payload-ovima) i kanonski string naziv (koristi se u `event` envelope polju i u `X-FastComments-Agent-Event` HTTP headeru).

| Naziv događaja | Enum | Aktivira se kada |
|---|---|---|
| `trigger.succeeded` | 0 | Agent run se završava sa statusom `SUCCESS`. |
| `trigger.failed` | 1 | Agent run se završava sa statusom `ERROR`. |
| `approval.requested` | 2 | Odobrenje se stavlja u red u `PENDING` stanju. |
| `approval.decided` | 3 | Odobrenje prelazi u `APPROVED`, `REJECTED` ili `EXECUTION_FAILED`. |

### `trigger.succeeded`

Aktivira se nakon što se agentov run završi bez greške. `data` polje payload-a uključuje:

- `triggerId` - jedinstveni ID run-a.
- `triggerType` - [enum razloga okidanja](#triggers-overview) koji je pokrenuo run.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokeni potrošeni u ovom run-u.
- `wasDryRun` - true ako je agent bio u [dry-run režimu](#dry-run-mode).
- `actions` - niz `TenantAgentAction` zapisa (vidi [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - ako ih je okidač imao.

Ako run nije preduzeo nijednu akciju, `actions` niz je prazan - ovo je uspešan run "agent je odlučio da ne radi ništa", što je korisno znati.

### `trigger.failed`

Aktivira se kada run dođe do greške. Isti oblik payload-a kao `trigger.succeeded`, sa `status: 'ERROR'` i dodatnim `errorMessage` poljem koje opisuje šta je pošlo naopako. Moguće greške uključuju neuspehe LLM poziva, neuspehe dispatch-a alata i iscrpljivanje budžeta usred run-a.

`actions` i dalje može sadržati unose za pozive alata koji su završeni pre greške.

### `approval.requested`

Aktivira se u trenutku kada se odobrenje stavi u red u `PENDING` stanju. Payload uključuje:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - argumenti alata **prosleđeni doslovno** iz LLM poziva. Oblik je specifičan po alatu i nije stabilan javni ugovor - šema se može menjati kako se dodaju novi alati.
- `createdAt`.
- `justification`, `confidence` - ako ih je agent dostavio.
- `contextSnapshot` - kontekst komentara / stranice na koji se odobrenje odnosi.

Korisno za prosleđivanje odobrenja koja čekaju u chat ops kanal: Slack bot pretplaćen na `approval.requested` može objaviti akciju i obrazloženje u moderacioni kanal za pregled na prvi pogled.

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

- **Odobreno + uspešno izvršeno** -> `status: APPROVED`, `executedAt` postavljeno, `executionResult` je poruka o uspehu.
- **Odobreno + izvršitelj nije uspeo** -> `status: EXECUTION_FAILED`, `executedAt` postavljeno, `executionResult` opisuje neuspeh.
- **Odbijeno** -> `status: REJECTED`, `executedAt` je null, `executionResult` je null.

### Header

Svaka isporuka uključuje `X-FastComments-Agent-Event` HTTP header sa kanonskim string nazivom događaja (`trigger.succeeded`, itd.). Korisno ako je vaš endpoint jedan URL koji obrađuje više tipova događaja.

### Vidi takođe

- [Webhook Payloads](#webhook-payloads) za kompletne šeme payload-a po događaju.
- [Webhook Signing](#webhook-signing) za HMAC šemu.
- [Webhook Retries](#webhook-retries) za semantiku isporuke.