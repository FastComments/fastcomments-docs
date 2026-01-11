FastComments webhook захтеви укључују више механизама аутентификације ради безбедности.

## Заглавља која се шаљу

| Header | Description |
|--------|-------------|
| `token` | Ваш API Secret (ради назадне компатибилности) |
| `X-FastComments-Timestamp` | Unix временска ознака (у секундама) када је захтев потписан |
| `X-FastComments-Signature` | HMAC-SHA256 потпис података (payload) |

## Верификација HMAC потписа (препоручено)

Снажно препоручујемо верификацију HMAC потписа како бисте осигурали да су payload-ови вебхука аутентични и да нису биле измене.

**Формат потписа:** `sha256=<hex-encoded-signature>`

**Како се потпис израчунава:**
1. Concatenate: `timestamp + "." + JSON_payload_body`
2. Compute HMAC-SHA256 using your API Secret as the key
3. Hex-encode the result

### Пример верификације (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Потврдите да је временска ознака новија (у року од 5 минута)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Заштита од replay напада
    }

    // Потврдите потпис
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Пример верификације (Python)

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

    # Потврдите да је временска ознака новија
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Потврдите потпис
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Пример верификације (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Потврдите да је временска ознака новија (у року од 5 минута)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Потврдите потпис
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Наслеђе аутентификације

Хедер `token` који садржи ваш API Secret и даље се шаље ради назадне компатибилности. Међутим, препоручујемо миграцију на HMAC верификацију за побољшану безбедност јер штити од replay напада.