Postoje četiri tipa agent webhook događaja. Svaki događaj ima numeričku enum vrijednost (koristi se u payload-ima) i kanonsko string ime (koristi se u `event` polju omotača i u `X-FastComments-Agent-Event` HTTP zaglavlju).

| Ime događaja | Enum | Događa se kada |
|---|---|---|
| `trigger.succeeded` | 0 | Pokret agenta se završava sa statusom `SUCCESS`. |
| `trigger.failed` | 1 | Pokret agenta se završava sa statusom `ERROR`. |
| `approval.requested` | 2 | Zahtjev za odobrenjem je stavljeno u `PENDING` stanje. |
| `approval.decided` | 3 | Zahtjev za odobrenjem prelazi u `APPROVED`, `REJECTED`, ili `EXECUTION_FAILED`. |

### `trigger.succeeded`

Događa se nakon što se pokret agenta završi bez greške. `data` polje payload-a uključuje:

- `triggerId` - jedinstveni ID pokreta.
- `triggerType` - [enum razloga okidača](#triggers-overview) koji je pokrenuo izvršavanje.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokeni potrošeni u ovom pokretu.
- `wasDryRun` - true ako je agent bio u dry-run režimu.
- `actions` - niz `TenantAgentAction` zapisa (vidi [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - ako su postojali za okidač.

Ako pokret nije izvršio nijednu akciju, niz `actions` je prazan - ovo je uspješan pokret tipa "agent je odlučio da ne radi ništa", što je korisna informacija.

### `trigger.failed`

Događa se kada pokret završi greškom. Isti oblik payload-a kao za `trigger.succeeded`, sa `status: 'ERROR'` i dodatnim poljem `errorMessage` koje opisuje šta je pošlo po zlu. Moguće greške uključuju neuspjehe poziva LLM-a, neuspjehe u slanju alatu, i iscrpljivanje budžeta tokom pokreta.

`actions` i dalje može sadržavati zapise za pozive alata koji su završeni prije greške.

### `approval.requested`

Događa se u trenutku kada je zahtjev za odobrenjem stavljeno u `PENDING` stanje. Payload uključuje:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - argumenti alata **proslijeđeni doslovno** iz poziva LLM-a. Oblik je po-alatu i nije stabilan javni ugovor - šema se može promijeniti kako se dodaju novi alati.
- `createdAt`.
- `justification`, `confidence` - ako ih je agent dostavio.
- `contextSnapshot` - snimak konteksta komentara / stranice na koji se odobrenje odnosi.

Koristan za prosljeđivanje čekajućih odobrenja u chat ops kanal: Slack bot pretplaćen na `approval.requested` može objaviti radnju i obrazloženje u kanal za moderaciju radi brzog pregleda.

### `approval.decided`

Događa se kada zahtjev za odobrenjem više nije u `PENDING` stanju. Payload uključuje:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, ili `EXECUTION_FAILED`.
- `decidedBy` - ID korisnika moderatora koji je donio odluku.
- `decidedAt` - kada je donio odluku.
- `executedAt` - ako je APPROVED, kada je platforma izvršila odobrenu radnju.
- `executionResult` - ako je APPROVED, string koji opisuje rezultat izvršitelja.
- `contextSnapshot` - snimak konteksta komentara / stranice.

Ovaj događaj pokriva sve ishode odluke:

- **Odobreno + izvršeno bez greške** -> `status: APPROVED`, `executedAt` postavljen, `executionResult` je poruka o uspjehu.
- **Odobreno + izvršitelj nije uspio** -> `status: EXECUTION_FAILED`, `executedAt` postavljen, `executionResult` opisuje neuspjeh.
- **Odbijeno** -> `status: REJECTED`, `executedAt` je null, `executionResult` je null.

### Zaglavlje

Svaka dostava uključuje `X-FastComments-Agent-Event` HTTP zaglavlje sa kanonskim string imenom događaja (`trigger.succeeded`, itd.). Korisno ako vaš endpoint koristi jedan URL za rukovanje više tipova događaja.

### Pogledajte takođe

- [Webhook Payloads](#webhook-payloads) za potpune šeme payload-a po događaju.
- [Potpisivanje webhook-a](#webhook-signing) za HMAC šemu.
- [Pokušaji isporuke webhook-a](#webhook-retries) za semantiku dostave.