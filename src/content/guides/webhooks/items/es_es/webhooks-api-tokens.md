---
Las solicitudes webhook de FastComments incluyen múltiples mecanismos de autenticación por motivos de seguridad.

## Encabezados enviados

| Header | Description |
|--------|-------------|
| `token` | Tu API Secret (para compatibilidad con versiones anteriores) |
| `X-FastComments-Timestamp` | Marca de tiempo Unix (segundos) cuando se firmó la solicitud |
| `X-FastComments-Signature` | Firma HMAC-SHA256 de la carga útil |

## Verificación de firma HMAC (Recomendado)

Recomendamos encarecidamente verificar la firma HMAC para garantizar que las cargas útiles de los webhooks sean auténticas y no hayan sido manipuladas.

**Formato de la firma:** `sha256=<hex-encoded-signature>`

**Cómo se calcula la firma:**
1. Concatenar: `timestamp + "." + JSON_payload_body`
2. Calcular HMAC-SHA256 usando tu API Secret como clave
3. Codificar el resultado en hexadecimal

### Ejemplo de verificación (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Verificar que la marca de tiempo sea reciente (dentro de 5 minutos)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Prevención de ataques de repetición
    }

    // Verificar firma
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Ejemplo de verificación (Python)

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

    # Verificar que la marca de tiempo sea reciente
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Verificar firma
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Ejemplo de verificación (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Verificar que la marca de tiempo sea reciente (dentro de 5 minutos)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Verificar firma
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Autenticación heredada

El encabezado `token` que contiene tu API Secret todavía se envía por compatibilidad con versiones anteriores. Sin embargo, recomendamos migrar a la verificación HMAC para mejorar la seguridad, ya que protege contra ataques de repetición.

---