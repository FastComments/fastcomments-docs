FastComments webhookverzoeken bevatten meerdere authenticatiemechanismen voor beveiliging.

## Verzonden headers

| Header | Beschrijving |
|--------|-------------|
| `token` | Uw API Secret (voor achterwaartse compatibiliteit) |
| `X-FastComments-Timestamp` | Unix-timestamp (seconden) wanneer het verzoek werd ondertekend |
| `X-FastComments-Signature` | HMAC-SHA256-handtekening van de payload |

## HMAC-handtekeningverificatie (aanbevolen)

We raden sterk aan de HMAC-handtekening te verifiëren om ervoor te zorgen dat webhook-payloads authentiek zijn en niet zijn gewijzigd.

**Handtekeningformaat:** `sha256=<hex-encoded-signature>`

**Hoe de handtekening wordt berekend:**
1. Samenvoegen: `timestamp + "." + JSON_payload_body`
2. Bereken HMAC-SHA256 met uw API Secret als sleutel
3. Hex-encodeer het resultaat

### Voorbeeldverificatie (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Controleer of de timestamp recent is (binnen 5 minuten)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Ter voorkoming van replay-aanvallen
    }

    // Verifieer handtekening
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Voorbeeldverificatie (Python)

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

    # Controleer of de timestamp recent is
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Verifieer handtekening
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Voorbeeldverificatie (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Controleer of de timestamp recent is (binnen 5 minuten)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Verifieer handtekening
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Verouderde authenticatie

De `token`-header die uw API Secret bevat, wordt nog steeds verzonden voor achterwaartse compatibiliteit. We raden echter aan over te stappen op HMAC-verificatie voor verbeterde beveiliging, omdat dit beschermt tegen replay-aanvallen.