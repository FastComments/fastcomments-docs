---
Agent webhooks forsøger igen ved fejl. Levering er **fire-and-forget fra agentens perspektiv** - en mislykket levering blokerer ikke agentens udførsel eller ruller nogen handlinger tilbage - og en kø + cron håndterer genforsøg asynkront.

### Kømodel

Hver begivenhed bliver sat i kø **én gang per matchende webhook**. Så hvis du har tre webhooks tilmeldt `trigger.succeeded` for en given agent + domæne, placerer platformen tre leveringer i kø; hver leveres og genforsøges uafhængigt. En fejl på én webhook påvirker aldrig de andre.

### Hvad der genforsøges

En levering genforsøges når:

- HTTP-forespørgslen **ikke fuldføres** (DNS-fejl, forbindelse afvist, timeout).
- HTTP-responskoden er enhver non-2xx status, som ikke er i den konfigurerede **Statuskoder uden genforsøg**-liste.

En levering **genforsøges ikke** når:

- Responskoden er `2xx` (succes).
- Responskoden er i den konfigurerede **Statuskoder uden genforsøg**-liste. Som standard er denne liste tom - enhver non-2xx genforsøges.

### Konfiguration af statuskoder uden genforsøg

Webhook-konfigurationsformularen har et felt **Statuskoder uden genforsøg** (multi-værdi). Almindelige indtastninger:

- `410` - Gone. Dit endpoint er permanent flyttet eller ressourcen er væk. At genprøve spilder bare båndbredde for begge parter.
- `422` - Unprocessable Entity. Dit endpoint forstod payloaden, men vurderede den som ugyldig. Genforsøg med samme payload vil give samme svar.
- `400` - Bad Request, i samme ånd.

At tilføje en kode her betyder: når endpointet returnerer den, markér leveringen som failed-terminal og stop genforsøg.

### Genforsøgsplan

En baggrundsarbejder kører hvert par sekunder og behandler alle leveringer, hvis næste forsøgstidspunkt er overskredet.

Efter hver fejl skubbes næste-forsøgstidspunktet frem med **lineær backoff**: ventetiden vokser som `60 seconds * attempt count` (så forsøg 1 venter 1 minut, forsøg 2 venter 2 minutter, og så videre).

Efter 99 mislykkede forsøg (eller 3 i lokal udvikling) opgives leveringen og fjernes fra køen. Leveringslogposter bevares dog og forbliver synlige på [Webhook Delivery Logs](#webhook-logs) siden indtil de udløber.

### Idempotens på din side

Fordi vi genforsøger, skal dit endpoint være idempotent. Den samme `triggerId` (eller `approvalId`) kan ankomme mere end én gang. Dit endpoint bør:

- Bruge en unik nøgle (`triggerId` for trigger-begivenheder, `approvalId` for approval-begivenheder) som et deduplikerings-token.
- Acceptere dubletleveringer yndefuldt (returner 200 anden gang).

Et ikke-idempotent endpoint vil til sidst dobbelthåndtere nogle leveringer, især under transiente nedbrud hvor en timeout genforsøger 30 sekunder senere, men den oprindelige forespørgsel faktisk lykkedes.

### Rækkefølge

Leveringer er **ikke strengt ordnede**. En `trigger.succeeded` og en downstream `approval.requested` (fra samme kørsel) kan ankomme i vilkårlig rækkefølge, hvis den ene genforsøger og den anden ikke gør. Dit endpoint bør ikke antage årsagsmæssig rækkefølge.

Hvis du har brug for rækkefølge, brug tidsstemplerne - `occurredAt` på konvolutten, plus trigger-/approval-`createdAt` i data-blokken - for at genskabe rækkefølgen hos dig.

### Oprydning

Leveringer fjernes fra køen så snart de enten lykkes eller rammer forsøgsgrænsen. Platformen beholder ikke terminalt fejlede leveringer i selve køen; det holdbare register over hvert forsøg ligger i [Webhook Delivery Logs](#webhook-logs) siden.

### Hvor du skal kigge, når genforsøg mislykkes

Siden [Webhook Delivery Logs](#webhook-logs) er stedet at se, hvorfor en webhook fejler. Almindelige årsager:

- **DNS-opløsning mislykkedes** - URL'en er forkert eller domænet er væk.
- **TLS-fejl** - dit endpoints certifikat er ugyldigt eller udløbet.
- **Forbindelse afvist / timeout** - dit endpoint er nede.
- **5xx-responser** - dit endpoint er oppe men fejler. Responsens indhold (afkortet) optages.
- **4xx-responser** - dit endpoint afviste payloaden. Hvis dette er tilsigtet, tilføj koden til **Statuskoder uden genforsøg**.

### Sæt en fejlbehæftet webhook på pause

Hvis en webhook konsekvent fejler, er den reneste løsning at slette den (eller midlertidigt rydde dens begivenhedsabonnementsliste). Platformen deaktiverer ikke automatisk fejlede webhooks - de bliver ved med at genforsøge, indtil leveringen opgives.

---