Запросы вебхуков FastComments включают несколько механизмов аутентификации для безопасности.

## Headers Sent

| Header | Description |
|--------|-------------|
| `token` | Ваш API Secret (для обратной совместимости) |
| `X-FastComments-Timestamp` | Unix-временная метка (в секундах), когда запрос был подписан |
| `X-FastComments-Signature` | HMAC-SHA256 подпись полезной нагрузки |

## HMAC Signature Verification (Recommended)

Мы настоятельно рекомендуем проверять HMAC-подпись, чтобы убедиться, что полезные данные вебхука подлинны и не были изменены.

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

    // Проверяет, что метка времени свежая (в пределах 5 минут)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Защита от повторного воспроизведения (replay-атаки)
    }

    // Проверка подписи
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

    # Проверяет, что метка времени свежая
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Проверка подписи
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

    // Проверяет, что метка времени свежая (в пределах 5 минут)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Проверка подписи
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Legacy Authentication

The `token` header containing your API Secret is still sent for backwards compatibility. However, we recommend migrating to HMAC verification for improved security as it protects against replay attacks.

---