Every agent webhook is signed with HMAC-SHA256 using your tenant's API secret. The same signing scheme is used for FastComments' comment webhooks - if you have already integrated those, the agent webhooks reuse the same signature header and verification flow.

### Dlaczego podpisywanie

Bez podpisu, atakujący, który zna URL twojego webhooka, mógłby wykonać POST z sfałszowanymi zdarzeniami wyglądającymi na pochodzące z FastComments. Podpisywanie oznacza, że twój endpoint może zweryfikować każde dostarczenie jako autentyczne zanim na nie zareaguje.

### Jak działają podpisy

For each delivery:

1. The platform looks up the API secret for the tenant + matched domain (see [Webhooks Overview](#webhooks-overview)).
2. It emits the current Unix timestamp (in milliseconds) in the `X-FastComments-Timestamp` header.
3. It computes `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (Stripe-style) and emits the result as `sha256=<hex>` in the `X-FastComments-Signature` header.
4. Your endpoint reads the timestamp header, recomputes the HMAC over `${timestamp}.${body}` it received, compares to the `sha256=<hex>` value in the signature header, and rejects mismatches.

The body that is signed is the **exact bytes** the platform sent, prefixed with `${timestamp}.` - your verifier must use the raw request body, not a re-serialized JSON string (key ordering and whitespace would otherwise differ).

### API secret

The same API Secret used by [comment webhooks](/guide-webhooks.html). It is per (tenant, domain) and managed in your tenant's API settings. If you rotate the secret, you should re-deploy your verifier to read the new value before the next delivery.

When the platform finds **no API secret** for the matched domain, the delivery does not happen. The webhook log records the failure with reason "no API secret".

### Verification example (Node.js)

[inline-code-attrs-start title = 'Przykład weryfikacji podpisu webhooka'; type='javascript' inline-code-attrs-end]
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

Użyj `timingSafeEqual` zamiast `===`, aby uniknąć ujawniania podpisu przez ataki kanału czasowego.

### Co znajduje się w podpisanym ciele

The full envelope plus the event-specific `data` block. See [Webhook Payloads](#webhook-payloads).

### Zalecenia

- **Weryfikuj przy każdej dostawie.** Jeśli twój endpoint akceptuje niepodpisane żądania, nie masz gwarancji integralności.
- **Odrzucaj przy niezgodności podpisu.** Zwróć 401 lub 403; nie zwracaj 200 OK przy złym podpisie, bo zamaskujesz ataki w logach dostaw.
- **Używaj HTTPS.** Podpisy chronią integralność; TLS chroni poufność (zarówno twojego sekretu, jak i tekstu komentarza w payloadzie).
- **Rotuj sekrety** gdy członkowie zespołu z dostępem odchodzą, lub zgodnie z harmonogramem.

### Ochrona przed atakami powtórzeniowymi

Signing alone does not prevent replay attacks - an attacker who captured a real signed delivery can re-send it. Replay protection is up to your endpoint:

- Use the `occurredAt` envelope field and reject deliveries older than, say, 5 minutes.
- Use the `triggerId` or `approvalId` as a dedup key - if you have already processed it, ignore the duplicate.

### Zobacz także

- [Przegląd webhooków](#webhooks-overview).
- [Payloady webhooków](#webhook-payloads).
- The main [Przewodnik po webhookach](/guide-webhooks.html) for the broader signing infrastructure.