Every agent webhook is signed with HMAC-SHA256 using your tenant's API secret. The same signing scheme is used for FastComments' comment webhooks - if you have already integrated those, the agent webhooks reuse the same signature header and verification flow.

### Why signing

Without a signature, an attacker who knows your webhook URL could POST forged events that look like they came from FastComments. Signing means your endpoint can verify each delivery is authentic before acting on it.

### How signatures work

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

[inline-code-attrs-start title = 'דוגמת אימות חתימת Webhook'; type='javascript' inline-code-attrs-end]
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

Use `timingSafeEqual` rather than `===` to avoid timing-channel leaks of the signature.

### What's in the signed body

The full envelope plus the event-specific `data` block. See [Webhook Payloads](#webhook-payloads).

### Recommendations

- **Verify on every delivery.** If your endpoint accepts unsigned requests, you have no integrity guarantee.
- **Reject on signature mismatch.** Return 401 or 403; do not 200 OK on a bad signature, or you will mask attacks in your delivery logs.
- **Use HTTPS.** Signatures protect integrity; TLS protects confidentiality (both your secret and the comment text in the payload).
- **Rotate secrets** when team members with access leave, or on a schedule.

### Replay protection

Signing alone does not prevent replay attacks - an attacker who captured a real signed delivery can re-send it. Replay protection is up to your endpoint:

- Use the `occurredAt` envelope field and reject deliveries older than, say, 5 minutes.
- Use the `triggerId` or `approvalId` as a dedup key - if you have already processed it, ignore the duplicate.

### See also

- [Webhooks Overview](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- The main [Webhooks guide](/guide-webhooks.html) for the broader signing infrastructure.