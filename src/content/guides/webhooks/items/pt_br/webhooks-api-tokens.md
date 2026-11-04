As solicitações de webhook do FastComments incluem múltiplos mecanismos de autenticação para segurança.

## Cabeçalhos Enviados

| Cabeçalho | Descrição |
|--------|-------------|
| `token` | Seu API Secret (para compatibilidade retroativa) |
| `X-FastComments-Timestamp` | Timestamp Unix (segundos) de quando a requisição foi assinada |
| `X-FastComments-Signature` | Assinatura HMAC-SHA256 do payload |

## Verificação de Assinatura HMAC (Recomendado)

Recomendamos fortemente verificar a assinatura HMAC para garantir que os payloads do webhook sejam autênticos e não tenham sido adulterados.

**Formato da assinatura:** `sha256=<hex-encoded-signature>`

**Como a assinatura é calculada:**
1. Concatenar: `timestamp + "." + JSON_payload_body`
2. Calcular HMAC-SHA256 usando seu API Secret como chave
3. Codificar o resultado em hexadecimal

### Exemplo de Verificação (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Verify timestamp is recent (within 5 minutes)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Replay attack prevention
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

### Exemplo de Verificação (Python)

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

    # Verify timestamp is recent
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Verify signature
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Exemplo de Verificação (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Verify timestamp is recent (within 5 minutes)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Verify signature
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Autenticação Legada

O cabeçalho `token` contendo seu API Secret ainda é enviado para compatibilidade retroativa. No entanto, recomendamos migrar para a verificação HMAC para melhorar a segurança, pois ela protege contra ataques de replay.