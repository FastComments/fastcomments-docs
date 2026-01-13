FastComments webhook-forespørgsler indeholder flere autentificeringsmekanismer for sikkerhed.

## Sendte headere

| Header | Beskrivelse |
|--------|-------------|
| `token` | Din API Secret (for bagudkompatibilitet) |
| `X-FastComments-Timestamp` | Unix-tidsstempel (sekunder) da anmodningen blev signeret |
| `X-FastComments-Signature` | HMAC-SHA256-signatur af payloaden |

## HMAC-signaturverifikation (Anbefalet)

Vi anbefaler kraftigt at verificere HMAC-signaturen for at sikre, at webhook-payloads er autentiske og ikke er blevet manipuleret med.

**Signaturformat:** `sha256=<hex-encoded-signature>`

**Hvordan signaturen beregnes:**
1. Sammenkæd: `timestamp + "." + JSON_payload_body`
2. Beregn HMAC-SHA256 ved at bruge din API Secret som nøgle
3. Hex-enkodér resultatet

### Eksempel på verifikation (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Bekræft at tidsstemplet er nyligt (inden for 5 minutter)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Forebyggelse af replay-angreb
    }

    // Bekræft signatur
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Eksempel på verifikation (Python)

```python
import hmac
import hashlib
import time
import json

def verify_webhook_signature(headers, body, api_secret):
    timestamp = headers.get('X-FastComments-Timestamp')
    signature = headers.get('X-FastComments-Signature')

    if not timestamp or not signature:
        return False

    # Bekræft at tidsstemplet er nyligt
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Bekræft signatur
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Eksempel på verifikation (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Bekræft at tidsstemplet er nyligt (inden for 5 minutter)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Bekræft signatur
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Ældre autentificering

`token`-headeren, der indeholder din API Secret, sendes stadig for bagudkompatibilitet. Vi anbefaler dog at migrere til HMAC-verifikation for forbedret sikkerhed, da det beskytter mod replay-angreb.

---