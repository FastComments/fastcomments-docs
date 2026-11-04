Запросы вебхуков FastComments включают несколько механизмов аутентификации для обеспечения безопасности.

## Отправляемые заголовки

| Заголовок | Описание |
|--------|-------------|
| `token` | Ваш API Secret (для обратной совместимости) |
| `X-FastComments-Timestamp` | Unix-время (в секундах), когда запрос был подписан |
| `X-FastComments-Signature` | Подпись HMAC-SHA256 полезной нагрузки |

## Проверка подписи HMAC (рекомендуется)

Мы настоятельно рекомендуем проверять подпись HMAC, чтобы убедиться, что полезная нагрузка вебхука подлинная и не была изменена.

**Формат подписи:** `sha256=<hex-encoded-signature>`

**Как вычисляется подпись:**
1. Конкатенация: `timestamp + "." + JSON_payload_body`
2. Вычисление HMAC-SHA256 с использованием вашего API Secret в качестве ключа
3. Кодирование результата в hex

### Пример проверки (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Проверка, что временная метка свежая (в пределах 5 минут)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Защита от атак повторного воспроизведения
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

### Пример проверки (Python)

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

    # Проверка, что временная метка свежая
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

### Пример проверки (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Проверка, что временная метка свежая (в пределах 5 минут)
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

## Устаревшая аутентификация

Заголовок `token`, содержащий ваш API Secret, по-прежнему отправляется для обратной совместимости. Однако мы рекомендуем перейти на проверку HMAC для повышения безопасности, так как она защищает от атак повторного воспроизведения.