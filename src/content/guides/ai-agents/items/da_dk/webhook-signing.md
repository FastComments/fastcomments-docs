Hver agent-webhook er underskrevet med HMAC-SHA256 ved hjælp af din tenants API secret. Den samme underskrivningsmetode bruges til FastComments' kommentar-webhooks – hvis du allerede har integreret disse, genbruger agent-webhooks den samme signaturheader og verifikationsflow.

### Hvorfor underskrivelse

Uden en signatur kunne en angriber, som kender din webhook-URL, POSTe forfalskede events, der ser ud som om de kom fra FastComments. Underskrivelse betyder, at dit endpoint kan verificere, at hver levering er autentisk, før den handler på den.

### Hvordan signaturer fungerer

For hver levering:

1. Platformen slår API secret op for tenant + matchet domæne (se [Oversigt over webhooks](#webhooks-overview)).
2. Den udsender det aktuelle Unix-timestamp (i millisekunder) i headeren `X-FastComments-Timestamp`.
3. Den beregner `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (i Stripe-stil) og udsender resultatet som `sha256=<hex>` i headeren `X-FastComments-Signature`.
4. Dit endpoint læser timestamp-headeren, genberegner HMAC over `${timestamp}.${body}` som det modtog, sammenligner med `sha256=<hex>`-værdien i signatur-headeren og afviser uoverensstemmelser.

Body'en, der underskrives, er de **præcise bytes** platformen sendte, præficeret med `${timestamp}.` - din verifikator skal bruge den rå request-body, ikke en re-serialiseret JSON-streng (nøgleordning og mellemrum ville ellers være forskellige).

### API secret

Den samme API secret, der bruges af [kommentar-webhooks](/guide-webhooks.html). Den er per (tenant, domæne) og administreres i din tenants API-indstillinger. Hvis du roterer secret, bør du genudrulle din verifikator, så den læser den nye værdi før den næste levering.

Når platformen ikke finder **noget API secret** for det matchede domæne, sker leveringen ikke. Webhook-loggen registrerer fejlen med årsag "no API secret".

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

Brug `timingSafeEqual` i stedet for `===` for at undgå tidsmåls-kanal-lækager af signaturen.

### Hvad er i den signerede body

Hele envelopen plus event-specifikke `data`-blokken. Se [Webhook Payloads](#webhook-payloads).

### Anbefalinger

- **Verificér ved hver levering.** Hvis dit endpoint accepterer usignerede requests, har du ingen garanti for integritet.
- **Afvis ved signatur-uoverensstemmelse.** Returnér 401 eller 403; giv ikke 200 OK ved en forkert signatur, ellers vil du maskere angreb i dine leveringslogs.
- **Brug HTTPS.** Signaturer beskytter integriteten; TLS beskytter fortroligheden (både din secret og kommentarens tekst i payloaden).
- **Roter secrets** når teammedlemmer med adgang forlader, eller efter en plan.

### Genafspilningsbeskyttelse

Selv med underskrivelse forhindres genafspilningsangreb ikke automatisk – en angriber, der har opsnappet en ægte underskrevet levering, kan gensende den. Genafspilningsbeskyttelse håndteres af dit endpoint:

- Brug envelope-feltet `occurredAt` og afvis leveringer ældre end f.eks. 5 minutter.
- Brug `triggerId` eller `approvalId` som en de-dup nøgle – hvis du allerede har behandlet det, ignorer dubletten.

### Se også

- [Oversigt over webhooks](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Hovedguiden [Webhooks guide](/guide-webhooks.html) for den bredere underskrivningsinfrastruktur.