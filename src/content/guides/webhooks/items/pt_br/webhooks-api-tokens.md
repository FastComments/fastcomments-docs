FastComments webhook requests include multiple authentication mechanisms for security.

## Headers Sent

| Header | Description |
|--------|-------------|
| `token` | Seu API Secret (para compatibilidade com versões anteriores) |
| `X-FastComments-Timestamp` | Timestamp Unix (segundos) quando a requisição foi assinada |
| `X-FastComments-Signature` | Assinatura HMAC-SHA256 do payload |

## HMAC Signature Verification (Recommended)

Recomendamos fortemente verificar a assinatura HMAC para garantir que os payloads do webhook sejam autênticos e não tenham sido adulterados.

**Signature Format:** `sha256=<hex-encoded-signature>`

**How the signature is computed:**
1. Concatenate: `timestamp + "." + JSON_payload_body`
2. Compute HMAC-SHA256 using your API Secret as the key
3. Hex-encode the result

### Example Verification (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Verifica se o timestamp é recente (dentro de 5 minutos)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Prevenção contra replay
    }

    // Verifica a assinatura
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Example Verification (Python)

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

    # Verifica se o timestamp é recente
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Verifica a assinatura
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Example Verification (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Verifica se o timestamp é recente (dentro de 5 minutos)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Verifica a assinatura
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Legacy Authentication

O cabeçalho `token` contendo seu API Secret ainda é enviado para compatibilidade com versões anteriores. No entanto, recomendamos migrar para a verificação HMAC para melhorar a segurança, pois isso protege contra ataques de replay.