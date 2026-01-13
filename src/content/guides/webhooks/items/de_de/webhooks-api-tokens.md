FastComments Webhook-Anfragen enthalten mehrere Authentifizierungsmechanismen zur Sicherheit.

## Gesendete Header

| Header | Beschreibung |
|--------|-------------|
| `token` | Ihr API Secret (zur Abwärtskompatibilität) |
| `X-FastComments-Timestamp` | Unix-Zeitstempel (Sekunden), zu dem die Anfrage signiert wurde |
| `X-FastComments-Signature` | HMAC-SHA256-Signatur der Payload |

## HMAC-Signaturüberprüfung (empfohlen)

Wir empfehlen dringend, die HMAC-Signatur zu überprüfen, um sicherzustellen, dass Webhook-Payloads authentisch sind und nicht manipuliert wurden.

**Signaturformat:** `sha256=<hex-encoded-signature>`

**Wie die Signatur berechnet wird:**
1. Verketten: `timestamp + "." + JSON_payload_body`
2. Berechne HMAC-SHA256 mit Ihrem API Secret als Schlüssel
3. Das Ergebnis hexadezimal kodieren

### Beispielüberprüfung (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Überprüfe, ob der Zeitstempel aktuell ist (innerhalb von 5 Minuten)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Schutz vor Replay-Angriffen
    }

    // Überprüfe Signatur
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Beispielüberprüfung (Python)

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

    # Überprüfe, ob der Zeitstempel aktuell ist
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Überprüfe Signatur
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Beispielüberprüfung (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Überprüfe, ob der Zeitstempel aktuell ist (innerhalb von 5 Minuten)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Überprüfe Signatur
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Legacy-Authentifizierung

Der `token`-Header, der Ihr API Secret enthält, wird weiterhin zur Abwärtskompatibilität gesendet. Wir empfehlen jedoch, auf die HMAC-Überprüfung umzusteigen, um die Sicherheit zu verbessern, da diese vor Replay-Angriffen schützt.