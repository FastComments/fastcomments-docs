FastComments webhook requests include multiple authentication mechanisms for security.

## Відправлені заголовки

| Заголовок | Опис |
|-----------|------|
| `token` | Ваш API Secret (для зворотної сумісності) |
| `X-FastComments-Timestamp` | Unix-часова мітка (секунди) коли запит був підписаний |
| `X-FastComments-Signature` | HMAC-SHA256 підпис навантаження |

## Перевірка підпису HMAC (рекомендовано)

Ми настійно рекомендуємо перевіряти підпис HMAC, щоб переконатися, що навантаження вебхука є автентичними і не були підроблені.

**Формат підпису:** `sha256=<hex-encoded-signature>`

**Як обчислюється підпис:**
1. Конкатенувати: `timestamp + "." + JSON_payload_body`
2. Обчислити HMAC-SHA256, використовуючи ваш API Secret як ключ
3. Перетворити результат у шістнадцятковий формат

### Приклад перевірки (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Перевірити, чи часова мітка недавня (протягом 5 хвилин)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Запобігання повторній атаці
    }

    // Перевірити підпис
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Приклад перевірки (Python)

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

    # Перевірити, чи часова мітка недавня
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Перевірити підпис
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Приклад перевірки (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Перевірити, чи часова мітка недавня (протягом 5 хвилин)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Перевірити підпис
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Спадкова автентифікація

Заголовок `token`, що містить ваш API Secret, все ще надсилається для зворотної сумісності. Однак ми рекомендуємо перейти на перевірку HMAC для підвищеної безпеки, оскільки вона захищає від повторних атак.

---