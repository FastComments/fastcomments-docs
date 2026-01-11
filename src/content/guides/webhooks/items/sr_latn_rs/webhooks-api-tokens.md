FastComments webhook zahtevi uključuju više mehanizama autentifikacije radi bezbednosti.

## Zaglavlja koja se šalju

| Zaglavlje | Opis |
|--------|-------------|
| `token` | Vaš API Secret (za unazadnu kompatibilnost) |
| `X-FastComments-Timestamp` | Unix timestamp (sekunde) kada je zahtev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis payload-a |

## Verifikacija HMAC potpisa (Preporučeno)

Toplo preporučujemo verifikaciju HMAC potpisa kako biste osigurali da su webhook payload-ovi autentični i da nisu izmenjeni.

**Format potpisa:** `sha256=<hex-encoded-signature>`

**Kako se potpis izračunava:**
1. Sastavite: `timestamp + "." + JSON_payload_body`
2. Izračunajte HMAC-SHA256 koristeći vaš API Secret kao ključ
3. Hex-enkodirajte rezultat

### Primer verifikacije (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Proverite da je timestamp recentan (u roku od 5 minuta)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Prevencija replay napada
    }

    // Proverite potpis
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Primer verifikacije (Python)

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

    # Proverite da je timestamp recentan
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Proverite potpis
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Primer verifikacije (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Proverite da je timestamp recentan (u roku od 5 minuta)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Proverite potpis
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Nasleđena autentifikacija

Zaglavlje `token` koje sadrži vaš API Secret se i dalje šalje za unazadnu kompatibilnost. Međutim, preporučujemo migraciju na HMAC verifikaciju za poboljšanu bezbednost, jer štiti od replay napada.