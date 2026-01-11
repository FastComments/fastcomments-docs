FastComments webhook requests include multiple authentication mechanisms for security.

## Wysyłane nagłówki

| Nagłówek | Opis |
|--------|-------------|
| `token` | Twój API Secret (w celu zachowania zgodności wstecznej) |
| `X-FastComments-Timestamp` | Znacznik czasu Unix (sekundy), kiedy żądanie zostało podpisane |
| `X-FastComments-Signature` | Podpis HMAC-SHA256 ładunku |

## Weryfikacja podpisu HMAC (zalecane)

Zdecydowanie zalecamy weryfikację podpisu HMAC, aby upewnić się, że ładunki webhooków są autentyczne i nie zostały zmienione.

**Format podpisu:** `sha256=<hex-encoded-signature>`

**Jak obliczany jest podpis:**
1. Połącz: `timestamp + "." + JSON_payload_body`
2. Oblicz HMAC-SHA256 używając swojego API Secret jako klucza
3. Zakoduj wynik w hex

### Przykład weryfikacji (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Zweryfikuj, że znacznik czasu jest aktualny (w ciągu 5 minut)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Zapobieganie atakom powtórzeniowym
    }

    // Verify signature
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Przykład weryfikacji (Python)

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

    # Zweryfikuj, że znacznik czasu jest aktualny
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Zweryfikuj podpis
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Przykład weryfikacji (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Zweryfikuj, że znacznik czasu jest aktualny (w ciągu 5 minut)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Zweryfikuj podpis
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Starsze uwierzytelnianie

Nagłówek `token` zawierający Twój API Secret wciąż jest wysyłany dla zachowania zgodności wstecznej. Jednak zalecamy przejście na weryfikację HMAC dla lepszego bezpieczeństwa, ponieważ chroni ona przed atakami powtórzeniowymi.