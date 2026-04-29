Agent-webhooks proberen bij mislukking opnieuw. Aflevering is **fire-and-forget vanuit het perspectief van de agent** - een mislukte aflevering blokkeert de uitvoering van de agent niet en draait geen acties terug - en een wachtrij + cron zorgt asynchroon voor herhaalpogingen.

### Wachtrijmodel

Elk event wordt **eenmaal per matchende webhook** in de wachtrij gezet. Dus als je drie webhooks hebt geabonneerd op `trigger.succeeded` voor een bepaalde agent + domein, plaatst het platform drie afleveringen in de wachtrij; elk wordt onafhankelijk geleverd en opnieuw geprobeerd. Een fout bij één webhook beïnvloedt nooit de andere.

### Wat wordt opnieuw geprobeerd

Een aflevering wordt opnieuw geprobeerd wanneer:

- Het HTTP-verzoek **niet voltooit** (DNS-fout, verbinding geweigerd, time-out).
- De HTTP-responscode is een niet-`2xx` status die niet in de geconfigureerde **Geen-opnieuw-probeer statuscodes** lijst staat.

Een aflevering wordt **niet opnieuw geprobeerd** wanneer:

- De responscode `2xx` is (succes).
- De responscode in de geconfigureerde **Geen-opnieuw-probeer statuscodes** lijst staat. Standaard is deze lijst leeg - elk niet-`2xx` wordt opnieuw geprobeerd.

### No-retry-codes configureren

Het webhook-configuratieformulier heeft een veld **Geen-opnieuw-probeer statuscodes** (multi-value). Veelvoorkomende invoer:

- `410` - Gone. Je endpoint is permanent verplaatst of de resource bestaat niet meer. Opnieuw proberen verspilt alleen bandbreedte aan beide zijden.
- `422` - Unprocessable Entity. Je endpoint begreep de payload maar beschouwt deze als ongeldig. Opnieuw proberen met dezelfde payload levert hetzelfde antwoord op.
- `400` - Bad Request, in dezelfde geest.

Het toevoegen van een code hier betekent: wanneer het endpoint deze teruggeeft, markeer de aflevering als 'failed-terminal' en stop met opnieuw proberen.

### Schema voor opnieuw proberen

Een achtergrondworker draait elke paar seconden en verwerkt afleveringen waarvan de volgende pogingstijd is verstreken.

Na elke mislukking wordt de volgende pogingstijd vooruitgeschoven met **lineaire backoff**: de wachttijd groeit als `60 seconds * attempt count` (dus poging 1 wacht 1 minuut, poging 2 wacht 2 minuten, enzovoort).

Na 99 mislukte pogingen (of 3 in lokale ontwikkeling) wordt de aflevering opgegeven en uit de wachtrij verwijderd. De afleverlogboekvermeldingen blijven wel bestaan en zijn zichtbaar in de [Webhook afleveringslogboeken](#webhook-logs) pagina totdat ze verlopen.

### Idempotentie aan uw kant

Omdat we opnieuw proberen, moet uw endpoint **idempotent** zijn. Dezelfde `triggerId` (of `approvalId`) kan meer dan eens aankomen. Uw endpoint zou het volgende moeten doen:

- Gebruik een unieke sleutel (`triggerId` voor trigger-events, `approvalId` voor approval-events) als dedup-token.
- Accepteer dubbele afleveringen gracieus (geef de tweede keer 200 terug).

Een niet-idempotent endpoint zal uiteindelijk sommige afleveringen dubbel verwerken, vooral tijdens tijdelijke uitval waarbij één time-out 30 seconden later opnieuw probeert terwijl het oorspronkelijke verzoek eigenlijk toch geslaagd was.

### Volgorde

Afleveringen zijn **niet strikt geordend**. Een `trigger.succeeded` en een downstream `approval.requested` (van dezelfde run) kunnen in willekeurige volgorde aankomen als de ene opnieuw probeert en de andere niet. Uw endpoint mag geen causale volgorde veronderstellen.

Als u volgorde nodig heeft, gebruik de tijdstempels - `occurredAt` op de envelop, plus de trigger/approval `createdAt` in het data-blok - om de volgorde aan uw kant te reconstrueren.

### Opschoning

Afleveringen worden uit de wachtrij verwijderd zodra ze ofwel slagen of het pogingcap bereiken. Het platform bewaart terminal-mislukte afleveringen niet in de wachtrij zelf; het duurzame logboek van elke poging bevindt zich in de [Webhook afleveringslogboeken](#webhook-logs) pagina.

### Waar te kijken wanneer herhaalpogingen mislukken

De [Webhook afleveringslogboeken](#webhook-logs) pagina is de plek om te zien waarom een webhook faalt. Veelvoorkomende oorzaken:

- **DNS-resolutiefout** - de URL is verkeerd of het domein bestaat niet (meer).
- **TLS-fouten** - het certificaat van je endpoint is ongeldig of verlopen.
- **Verbinding geweigerd / time-out** - je endpoint is offline.
- **`5xx` responses** - je endpoint is bereikbaar maar ervaart een fout. De response body (afgekapt) wordt vastgelegd.
- **`4xx` responses** - je endpoint wees de payload af. Als dit opzettelijk is, voeg dan de code toe aan **Geen-opnieuw-probeer statuscodes**.

### Een problematische webhook pauzeren

Als een webhook consequent faalt, is de schoonste oplossing om deze te verwijderen (of tijdelijk de lijst met event-abonnementen leeg te maken). Het platform schakelt foutieve webhooks niet automatisch uit - ze blijven opnieuw proberen totdat de aflevering opgegeven wordt.