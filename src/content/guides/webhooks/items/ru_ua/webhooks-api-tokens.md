FastComments webhook requests include multiple authentication mechanisms for security.

## Отправляемые заголовки

| Заголовок | Описание |
|--------|-------------|
| `token` | Ваш API Secret (для обратной совместимости) |
| `X-FastComments-Timestamp` | Unix-временная метка (в секундах), когда запрос был подписан |
| `X-FastComments-Signature` | Подпись HMAC-SHA256 полезной нагрузки |

## HMAC Signature Verification (Recommended)

Мы настоятельно рекомендуем проверять подпись HMAC, чтобы убедиться, что полезные данные webhook подлинны и не были изменены.

**Signature Format:** `sha256=<hex-encoded-signature>`

**How the signature is computed:**
1. Объедините: `timestamp + "." + JSON_payload_body`
2. Вычислите HMAC-SHA256, используя ваш API Secret в качестве ключа
3. Преобразуйте результат в шестнадцатеричную строку

### Example Verification (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Проверить, что метка времени недавняя (в пределах 5 минут)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Предотвращение атак повторного воспроизведения
    }

    // Проверить подпись
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

    # Проверить, что метка времени недавняя
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Проверить подпись
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

    // Проверить, что метка времени недавняя (в пределах 5 минут)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Проверить подпись
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Legacy Authentication

Заголовок `token`, содержащий ваш API Secret, по-прежнему отправляется для обратной совместимости. Тем не менее, мы рекомендуем перейти на проверку HMAC для повышения безопасности, так как она защищает от атак повторного воспроизведения.