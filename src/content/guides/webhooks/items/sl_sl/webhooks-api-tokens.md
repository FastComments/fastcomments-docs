FastComments webhook zahtevki vsebujejo več mehanizmov za overjanje za zagotavljanje varnosti.

## Poslane glave

| Glava | Opis |
|--------|-------------|
| `token` | Vaš API skrivni ključ (za združljivost z starejšimi različicami) |
| `X-FastComments-Timestamp` | Unix časovni žig (v sekundah), ko je bil zahtevek podpisan |
| `X-FastComments-Signature` | HMAC-SHA256 podpis vsebine |

## Preverjanje HMAC podpisa (priporočeno)

Močno priporočamo preverjanje HMAC podpisa, da zagotovite, da so podatki webhooka avtentični in da z njimi ni bilo manipulirano.

**Oblika podpisa:** `sha256=<hex-encoded-signature>`

**Kako se izračuna podpis:**
1. Združi: `timestamp + "." + JSON_payload_body`
2. Izračunajte HMAC-SHA256 z vašim API skrivnim ključem kot ključem
3. Hex-kodirajte rezultat

### Primer preverjanja (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Preveri, ali je časovni žig svež (v 5 minutah)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Preprečevanje ponovitvenega napada
    }

    // Preveri podpis
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Primer preverjanja (Python)

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

    # Preveri, ali je časovni žig svež
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Preveri podpis
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Primer preverjanja (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Preveri, ali je časovni žig svež (v 5 minutah)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Preveri podpis
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Zastarelo overjanje

Glava `token`, ki vsebuje vaš API skrivni ključ, se še vedno pošilja zaradi združljivosti z starejšimi različicami. Vendar priporočamo prehod na preverjanje preko HMAC zaradi boljše varnosti, saj ščiti pred ponovitvenimi napadi.

---