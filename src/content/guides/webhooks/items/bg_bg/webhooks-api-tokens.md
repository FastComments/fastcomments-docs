FastComments webhook заявките включват множество механизми за автентикация с цел сигурност.

## Изпращани хедъри

| Хедър | Описание |
|--------|-------------|
| `token` | Вашият API Secret (за обратна съвместимост) |
| `X-FastComments-Timestamp` | Unix времеви печат (в секунди), когато заявката е подписана |
| `X-FastComments-Signature` | HMAC-SHA256 подпис на payload данните |

## Проверка на HMAC подписа (препоръчително)

Силно препоръчваме да проверявате HMAC подписа, за да сте сигурни, че webhook payload данните са автентични и не са били подменени.

**Формат на подписа:** `sha256=<hex-encoded-signature>`

**Как се изчислява подписът:**
1. Конкатенация: `timestamp + "." + JSON_payload_body`
2. Изчисляване на HMAC-SHA256 с вашия API Secret като ключ
3. Hex-кодиране на резултата

### Примерна проверка (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Проверка дали времевият печат е скорошен (в рамките на 5 минути)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Предпазване от replay атаки
    }

    // Проверка на подписа
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Примерна проверка (Python)

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

    # Проверка дали времевият печат е скорошен
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Проверка на подписа
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Примерна проверка (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Проверка дали времевият печат е скорошен (в рамките на 5 минути)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Проверка на подписа
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Наследена автентикация

Хедърът `token`, съдържащ вашия API Secret, все още се изпраща за обратна съвместимост. Препоръчваме обаче да мигрирате към HMAC проверка за подобрена сигурност, тъй като тя предпазва от replay атаки.