FastComments webhook zahtjevi uključuju više mehanizama provjere autentičnosti radi sigurnosti.

## Zaglavlja koja se šalju

| Zaglavlje | Opis |
|--------|-------------|
| `token` | Vaš API Secret (za kompatibilnost unatrag) |
| `X-FastComments-Timestamp` | Unix vremenska oznaka (sekunde) kada je zahtjev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis payloada |

## Verifikacija HMAC potpisa (preporučeno)

Toplo preporučujemo provjeru HMAC potpisa kako biste osigurali da su webhook podaci autentični i da nisu izmijenjeni.

**Format potpisa:** `sha256=<hex-encoded-signature>`

**Kako se potpis izračunava:**
1. Spojite: `timestamp + "." + JSON_payload_body`
2. Izračunajte HMAC-SHA256 koristeći svoj API Secret kao ključ
3. Hex-kodirajte rezultat

### Primjer verifikacije (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Provjerite je li vremenska oznaka svježa (u roku od 5 minuta)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Sprječavanje replay napada
    }

    // Provjerite potpis
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Primjer verifikacije (Python)

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

    # Provjerite je li vremenska oznaka svježa
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Provjerite potpis
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Primjer verifikacije (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Provjerite je li vremenska oznaka svježa (u roku od 5 minuta)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Provjerite potpis
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Naslijeđena autentikacija

`token` zaglavlje koje sadrži vaš API Secret i dalje se šalje radi kompatibilnosti unatrag. Međutim, preporučujemo prelazak na HMAC provjeru radi poboljšane sigurnosti jer štiti od replay napada.

---