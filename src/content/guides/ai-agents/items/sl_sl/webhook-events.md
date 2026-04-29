---
Obstajajo štiri vrste agentnih webhook dogodkov. Vsak dogodek ima numerično vrednost enum (uporablja se v payloadih) in kanonično nizovno ime (uporablja se v polju ovojnice `event` in v HTTP glavi `X-FastComments-Agent-Event`).

| Ime dogodka | Enum | Sproži se, ko |
|---|---|---|
| `trigger.succeeded` | 0 | Zagon agenta se zaključi s statusom `SUCCESS`. |
| `trigger.failed` | 1 | Zagon agenta se zaključi s statusom `ERROR`. |
| `approval.requested` | 2 | Zahteva za odobritev je v čakalnem stanju `PENDING`. |
| `approval.decided` | 3 | Zahteva za odobritev preide v `APPROVED`, `REJECTED` ali `EXECUTION_FAILED`. |

### `trigger.succeeded`

Sproži se po tem, ko se zagon agenta zaključi brez napake. Polje `data` v payloadu vključuje:

- `triggerId` - edinstveni ID zagona.
- `triggerType` - [enum razloga sprožitve](#triggers-overview), ki je sprožil zagon.
- `status` - `SUCCESS` (niz).
- `tokensUsed` - število porabljenih tokenov pri tem zagonu.
- `wasDryRun` - true, če je bil agent v [načinu suhega zagona](#dry-run-mode).
- `actions` - polje zapisov `TenantAgentAction` (glej [Podatki webhooka](#webhook-payloads)).
- `commentId`, `url`, `urlId` - če jih je sprožilec imel.

Če zagon ni izvedel nobene akcije, je polje `actions` prazno - to je uspešen zagon tipa "agent se je odločil, da ne naredi nič", kar je koristno vedeti.

### `trigger.failed`

Sproži se, ko pride do napake pri zagonu. Oblika payloada je enaka kot pri `trigger.succeeded`, z `status: 'ERROR'` in dodatnim poljem `errorMessage`, ki opisuje, kaj je šlo narobe. Možne napake vključujejo odpovedi klicev LLM, napake pri sprožanju orodij in izčrpanje proračuna med izvajanjem.

`actions` lahko še vedno vsebuje vnose za klice orodij, ki so se zaključili pred napako.

### `approval.requested`

Sproži se v trenutku, ko je zahteva za odobritev postavljena v stanje `PENDING`. Payload vključuje:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - argumenti orodja, **posredovani dobesedno** iz klica LLM. Oblika je odvisna od posameznega orodja in ni stabilen javni pogodbeni vmesnik - shema se lahko spremeni, ko se dodajo nova orodja.
- `createdAt`.
- `justification`, `confidence` - če jih je agent navedel.
- `contextSnapshot` - kontekst komentarja / strani, na katerega se odobritev nanaša.

Uporabno za posredovanje čakajočih odobritev v kanal za chat ops: Slack bot, naročen na `approval.requested`, lahko objavi dejanje in utemeljitev v moderacijski kanal za hiter pregled.

### `approval.decided`

Sproži se, ko zahteva za odobritev zapusti stanje `PENDING`. Payload vključuje:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, ali `EXECUTION_FAILED`.
- `decidedBy` - ID uporabnika moderatorja, ki je odločil.
- `decidedAt` - kdaj je odločil.
- `executedAt` - če je APPROVED, kdaj je platforma izvršila odobreno dejanje.
- `executionResult` - če je APPROVED, niz, ki opisuje rezultat izvrševalca.
- `contextSnapshot` - kontekst komentarja / strani.

Ta dogodek zajema vse izide odločanja:

- **Odobreno + izvedeno brez napak** -> `status: APPROVED`, `executedAt` nastavljen, `executionResult` je sporočilo o uspehu.
- **Odobreno + izvrševalec ni uspel** -> `status: EXECUTION_FAILED`, `executedAt` nastavljen, `executionResult` opisuje napako.
- **Zavrnjeno** -> `status: REJECTED`, `executedAt` je null, `executionResult` je null.

### Header

Vsaka dostava vključuje HTTP glavo `X-FastComments-Agent-Event` z kanoničnim nizovnim imenom dogodka (`trigger.succeeded`, itd.). Uporabno, če je vaš endpoint ena URL, ki obravnava več vrst dogodkov.

### Glej tudi

- [Podatki webhooka](#webhook-payloads) za celotne sheme payloadov po dogodku.
- [Podpisovanje webhookov](#webhook-signing) za HMAC shemo.
- [Ponovna pošiljanja webhookov](#webhook-retries) za semantiko dostave.

---