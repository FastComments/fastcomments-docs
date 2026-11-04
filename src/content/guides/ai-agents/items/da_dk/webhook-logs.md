Hver agent-webhook har sin egen leveringslog. Den kan tilgås fra [webhook-listesiden](https://fastcomments.com/auth/my-account/ai-agents/webhooks) via knappen **Logs** på hver webhook-række.

### Hvad er på siden

En pagineret tabel med én række pr. leveringsforsøg:

| Kolonne | Betydning |
|---|---|
| When | Hvornår forsøget fandt sted. |
| Event | Hændelsestypen (`trigger.succeeded`, `approval.requested` osv.). |
| Status | Leveringsstatus. |
| StatusCode | HTTP-statuskoden returneret af dit endpoint, når tilgængelig. |
| Description | En kort beskrivelse af udfaldet. For mislykkede leveringer, hvor der ikke blev modtaget noget HTTP-svar, gemmes den underliggende fejlbesked som `{error: <message>}`. |

Siden understøtter kun paginering - der er ingen status-, hændelsestype- eller datointerval-filtre.

### Hvad du kan gøre fra loggene

- **Beslut, om en statuskode skal være i No-retry.** Hvis du ser dit endpoint returnere den samme `4xx` igen og igen, er det et stærkt signal om, at du vil tilføje den til **No-retry status codes**, så platformen holder op med at prøve igen.

### Fejlinformation

Når en levering fejler uden et HTTP-svar (DNS-fejl, forbindelse nægtet, timeout, TLS-fejl osv.), gemmes den rå fejlbesked som `{error: <message>}`. Platformen kategoriserer ikke disse i navngivne kategorier som `TIMEOUT` eller `DNS_ERROR` - læs fejlbeskeden direkte for at se, hvad der skete.

For HTTP-svar viser StatusCode-kolonnen koden returneret af dit endpoint. Almindelige tilfælde:

- **Alle forsøg: `401` eller `403`** - dit endpoint afviser signaturen. Tjek, at du beregner HMAC over `${timestamp}.${body}` og bruger den rigtige hemmelighed. Se [Webhook Signing](#webhook-signing).
- **Alle forsøg: `422`** - dit endpoint anser payloaden for ugyldig. Enten ret dit endpoint, eller tilføj `422` til **No-retry status codes**, hvis afvisningen er forventet for visse hændelser.

### Pr. levering-kontekst

Hver logpost indeholder:

- `webhookId` - hvilken webhook-konfiguration, der producerede denne levering.
- `agentId` - hvilken agent leveringen handler om.
- `triggerId` eller `approvalId` - den underliggende post.
- `domain` - det matchede domæne.

Du kan bruge disse til at koble en mislykket levering til den kørsel, den relaterer sig til i [Run History](#run-history).

### Opbevaring

`AgentSyncLog`-poster har en flad 1-års TTL på `createdAt` uanset udfald - vellykkede og mislykkede leveringer opbevares i samme tidsrum. Efter opbevaringsperioden er logposten væk.

Hvis du har brug for langsigtet revisionssporing, er det bæredygtige mønster: lad **endpointet selv** gemme de leveringer, det modtager. Det kobler dit revisionslog fra platformens opbevaringspolitik.

### Test-send

Webhook-konfigurationsformularens **Test send**-knap skriver en falsk levering ind i samme logtabel, så du kan verificere forbindelsen ende-til-ende, før du stoler på rigtige hændelser. Testleveringer er tydeligt mærket i loggen, så de ikke forurener produktionsfejlstatistikker.

### Se også

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) for retry-semantikken, der driver disse logge.
- [Webhook Signing](#webhook-signing) for, hvordan du verificerer på dit endpoint.