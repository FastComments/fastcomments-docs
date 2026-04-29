Elke agent-webhook is ondertekend met HMAC-SHA256 met behulp van het API-secret van uw tenant. Hetzelfde ondertekeningsschema wordt gebruikt voor FastComments' comment-webhooks — als u die al hebt geïntegreerd, hergebruiken de agent-webhooks dezelfde handtekening-header en verificatiestroom.

### Waarom ondertekening

Zonder een handtekening zou een aanvaller die uw webhook-URL kent vervalste events kunnen POSTen die eruitzien alsof ze van FastComments komen. Ondertekenen betekent dat uw endpoint elke aflevering kan verifiëren als authentiek voordat u erop reageert.

### Hoe handtekeningen werken

Voor elke aflevering:

1. Het platform zoekt het API-secret op voor de tenant + gematchte domein (zie [Webhooks Overview](#webhooks-overview)).
2. Het geeft de huidige Unix-timestamp (in milliseconden) uit in de `X-FastComments-Timestamp` header.
3. Het berekent `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (Stripe-stijl) en stuurt het resultaat als `sha256=<hex>` in de `X-FastComments-Signature` header.
4. Uw endpoint leest de timestamp-header, berekent de HMAC opnieuw over `${timestamp}.${body}` die het ontving, vergelijkt met de `sha256=<hex>` waarde in de signature-header en wijst mismatches af.

Het body dat is ondertekend is de **exacte bytes** die het platform verzond, voorafgegaan door `${timestamp}.` - uw verifier moet de raw request body gebruiken, niet een opnieuw-geserializeerde JSON-string (key-volgorde en witruimte zouden anders verschillen).

### API-secret

Hetzelfde API-secret dat door [commentaar-webhooks](/guide-webhooks.html) wordt gebruikt. Het is per (tenant, domain) en wordt beheerd in de API-instellingen van uw tenant. Als u het secret roteert, moet u uw verifier opnieuw uitrollen zodat deze de nieuwe waarde leest voordat de volgende aflevering plaatsvindt.

Wanneer het platform **geen API secret** vindt voor het gematchte domein, vindt de aflevering niet plaats. Het webhook-log registreert de fout met reden "no API secret".

### Verificatievoorbeeld (Node.js)

[inline-code-attrs-start title = 'Voorbeeld: verificatie van webhook-handtekening'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

Gebruik `timingSafeEqual` in plaats van `===` om timing-kanaallekken van de handtekening te voorkomen.

### Wat staat er in het ondertekende berichtlichaam

De volledige envelope plus het event-specifieke `data` block. Zie [Webhook-payloads](#webhook-payloads).

### Aanbevelingen

- **Controleer bij elke aflevering.** Als uw endpoint onondertekende verzoeken accepteert, heeft u geen integriteitsgarantie.
- **Weiger bij afwijkende handtekening.** Geef 401 of 403 terug; stuur geen 200 OK bij een slechte handtekening, anders maskeert u aanvallen in uw leveringslogboeken.
- **Gebruik HTTPS.** Handtekeningen beschermen integriteit; TLS beschermt vertrouwelijkheid (zowel uw secret als de comment-tekst in de payload).
- **Roteer geheimen** wanneer teamleden met toegang vertrekken, of volgens een schema.

### Bescherming tegen herhalingsaanvallen

Alleen ondertekenen voorkomt geen replay-aanvallen — een aanvaller die een echte ondertekende aflevering heeft vastgelegd kan deze opnieuw verzenden. Replay-bescherming is de verantwoordelijkheid van uw endpoint:

- Gebruik het `occurredAt` envelope-veld en wijs afleveringen die ouder zijn dan bijvoorbeeld 5 minuten af.
- Gebruik de `triggerId` of `approvalId` als dedup-key — als u deze al hebt verwerkt, negeer dan het duplicaat.

### Zie ook

- [Webhooks Overview](#webhooks-overview).
- [Webhook-payloads](#webhook-payloads).
- De hoofdgids [Webhooks guide](/guide-webhooks.html) voor de bredere ondertekeningsinfrastructuur.