FastComments webhook requests include multiple authentication mechanisms for security.

## Intestazioni inviate

| Intestazione | Descrizione |
|--------|-------------|
| `token` | Il tuo API Secret (per compatibilità con le versioni precedenti) |
| `X-FastComments-Timestamp` | Timestamp Unix (secondi) quando la richiesta è stata firmata |
| `X-FastComments-Signature` | Firma HMAC-SHA256 del payload |

## Verifica della firma HMAC (Consigliata)

Raccomandiamo vivamente di verificare la firma HMAC per assicurarsi che i payload dei webhook siano autentici e non siano stati manomessi.

**Formato della firma:** `sha256=<hex-encoded-signature>`

**Come viene calcolata la firma:**
1. Concatena: `timestamp + "." + JSON_payload_body`
2. Calcola HMAC-SHA256 usando il tuo API Secret come chiave
3. Codifica il risultato in esadecimale

### Esempio di verifica (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Verifica che il timestamp sia recente (entro 5 minuti)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Prevenzione di attacchi di replay
    }

    // Verifica della firma
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Esempio di verifica (Python)

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

    # Verifica che il timestamp sia recente
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Verifica la firma
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Esempio di verifica (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Verifica che il timestamp sia recente (entro 5 minuti)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Verifica la firma
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Autenticazione legacy

L'intestazione `token` contenente il tuo API Secret viene ancora inviata per compatibilità con le versioni precedenti. Tuttavia, consigliamo di migrare alla verifica HMAC per una sicurezza migliorata poiché protegge dagli attacchi di replay.