Hver agent-webhook er signeret med HMAC-SHA256 ved brug af din tenants API-hemmelighed. Det samme signeringsskema bruges til FastComments' kommentar-webhooks - hvis du allerede har integreret disse, genbruger agent-webhooks det samme signaturheader og verifikationsflow.

### Hvorfor signering

Uden en signatur kunne en angriber, der kender din webhook-URL, POST'e forfalskede hændelser, der ser ud til at komme fra FastComments. Signering betyder, at dit endpoint kan verificere, at hver levering er autentisk, før der handles på den.

### Hvordan signaturer fungerer

For hver levering:

1. Platformen slår API-hemmeligheden op for tenant + matchet domæne (se [Webhooks Overview](#webhooks-overview)).
2. Den udsender det aktuelle Unix-tidsstempel (i millisekunder) i `X-FastComments-Timestamp`-headeren.
3. Den beregner `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (Stripe-stil) og udsender resultatet som `sha256=<hex>` i `X-FastComments-Signature`-headeren.
4. Dit endpoint læser tidsstempel-headeren, genberegner HMAC over `${timestamp}.${body}`, som det modtog, sammenligner med `sha256=<hex>`-værdien i signatur-headeren og afviser uoverensstemmelser.

Den krop, der signeres, er de **nøjagtige bytes**, som platformen sendte, præfikset med `${timestamp}.` - din verifikator skal bruge den rå request body, ikke en re-serialiseret JSON-streng (nøglerækkefølge og mellemrum ville ellers afvige).

### API-hemmelighed

Den samme API Secret, der bruges af [kommentar-webhooks](/guide-webhooks.html). Den er pr. (tenant, domæne) og administreres i din tenants API-indstillinger. Hvis du roterer hemmeligheden, bør du gen-udrulle din verifikator, så den læser den nye værdi før næste levering.

Når platformen ikke finder **nogen API-hemmelighed** for det matchede domæne, sker leveringen ikke. Webhook-loggen registrerer fejlen med årsagen "no API secret".

### Verifikationseksempel (Node.js)

[inline-code-attrs-start title = 'Eksempel på verifikation af webhook-signatur'; type='javascript' inline-code-attrs-end]
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

Brug `timingSafeEqual` i stedet for `===` for at undgå timing-kanal-lækager af signaturen.

### Hvad der er i den signerede krop

Den fulde envelope plus den hændelsesspecifikke `data`-blok. Se [Webhook Payloads](#webhook-payloads).

### Anbefalinger

- **Verificér ved hver levering.** Hvis dit endpoint accepterer usignerede requests, har du ingen integritetsgaranti.
- **Afvis ved signatur-uoverensstemmelse.** Returnér 401 eller 403; returnér ikke 200 OK ved en dårlig signatur, ellers vil du skjule angreb i dine leveringslogs.
- **Brug HTTPS.** Signaturer beskytter integritet; TLS beskytter fortrolighed (både din hemmelighed og kommentarteksten i payloaden).
- **Rotér hemmeligheder**, når teammedlemmer med adgang forlader virksomheden, eller efter en tidsplan.

### Beskyttelse mod replay

Signering alene forhindrer ikke replay-angreb - en angriber, der har fanget en ægte signeret levering, kan sende den igen. Beskyttelse mod replay er dit endpoints ansvar:

- Brug `occurredAt`-feltet i envelope og afvis leveringer, der er ældre end f.eks. 5 minutter.
- Brug `triggerId` eller `approvalId` som dedup-nøgle - hvis du allerede har behandlet den, ignorér duplikatet.

### Se også

- [Webhooks Overview](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Den primære [Webhooks guide](/guide-webhooks.html) for den bredere signeringsinfrastruktur.