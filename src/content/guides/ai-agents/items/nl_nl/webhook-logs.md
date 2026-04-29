Elke agent-webhook heeft een eigen afleveringslog. Te bereiken vanaf de [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) via de **Logboeken**-knop op elke webhookrij.

### Wat er op de pagina staat

Een gepagineerde tabel met één rij per afleveringspoging:

| Column | Meaning |
|---|---|
| Wanneer | Wanneer de poging plaatsvond. |
| Gebeurtenis | Het type gebeurtenis (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | De afleveringsstatus. |
| StatusCode | De HTTP-statuscode die door je endpoint is geretourneerd, indien beschikbaar. |
| Beschrijving | Een korte beschrijving van de uitkomst. Voor mislukte leveringen waarbij geen HTTP-respons is ontvangen, wordt het onderliggende foutbericht opgeslagen als `{error: <message>}`. |

De pagina ondersteunt alleen paginering - er zijn geen filters voor status, gebeurtenistype of datumbereik.

### Wat je vanuit de logs kunt doen

- **Bepalen of een statuscode in Geen-herhaalpoging moet staan.** Als je ziet dat je endpoint steeds dezelfde `4xx` retourneert, is dat een sterk signaal dat je deze wilt toevoegen aan **Statuscodes zonder herhaalpoging** zodat het platform stopt met opnieuw proberen.

### Foutinformatie

Wanneer een aflevering faalt zonder een HTTP-respons (DNS failure, connection refused, timeout, TLS error, etc.), wordt het ruwe foutbericht vastgelegd als `{error: <message>}`. Het platform categoriseert deze niet in benoemde buckets zoals `TIMEOUT` of `DNS_ERROR` - lees het foutbericht rechtstreeks om te zien wat er gebeurd is.

Voor HTTP-responses toont de StatusCode-kolom de code die door je endpoint is geretourneerd. Veelvoorkomende gevallen:

- **Alle pogingen: `401` of `403`** - je endpoint wijst de handtekening af. Controleer of je de HMAC berekent over `${timestamp}.${body}` en de juiste secret gebruikt. Zie [Webhook Signing](#webhook-signing).
- **Alle pogingen: `422`** - je endpoint denkt dat de payload ongeldig is. Ofwel los je je endpoint op, of voeg `422` toe aan **Statuscodes zonder herhaalpoging** als de afwijzing voor sommige gebeurtenissen verwacht wordt.

### Context per levering

Elke logvermelding bevat:

- `webhookId` - welke webhookconfiguratie deze aflevering heeft geproduceerd.
- `agentId` - over welke agent de aflevering gaat.
- `triggerId` of `approvalId` - het onderliggende record.
- `domain` - het gematchte domein.

Je kunt deze gebruiken om een mislukte aflevering te correleren met de run waar deze betrekking op heeft in [Uitvoeringsgeschiedenis](#run-history).

### Bewaring

`AgentSyncLog`-vermeldingen hebben een vaste TTL van 1 jaar op `createdAt`, ongeacht de uitkomst - succesvolle en mislukte leveringen worden even lang bewaard. Na de bewaartermijn is de logvermelding verdwenen.

Als je langdurige audit nodig hebt, is het duurzame patroon: laat het **endpoint zelf** de leveringen die het ontvangt, persistent opslaan. Dat ontkoppelt je auditlog van het retentiebeleid van het platform.

### Test verzenden

De **Test verzenden**-knop in het webhook-configuratieformulier schrijft een nepaflevering naar dezelfde logtabel zodat je de end-to-end connectiviteit kunt verifiëren voordat je op echte gebeurtenissen vertrouwt. Testleveringen zijn duidelijk gelabeld in de log zodat ze de productiestatistieken voor falen niet vervuilen.

### Zie ook

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) voor de retry-semantiek die deze logs aanstuurt.
- [Webhook Signing](#webhook-signing) voor hoe je op je endpoint kunt verifiëren.