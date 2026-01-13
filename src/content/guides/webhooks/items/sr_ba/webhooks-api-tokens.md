FastComments webhook захтјеви укључују више механизама аутентификације за безбједност.

## Заглавља која се шаљу

| Header | Description |
|--------|-------------|
| `token` | Ваш API Secret (ради повратне компатибилности) |
| `X-FastComments-Timestamp` | Unix временска ознака (у секундама) када је захтјев потписан |
| `X-FastComments-Signature` | HMAC-SHA256 потпис садржаја |

## Верификација HMAC потписа (Препоручено)

Снажно препоручујемо верификацију HMAC потписа како бисте били сигурни да су подаци webhook-а аутентични и да им није промијењен садржај.

**Формат потписа:** `sha256=<hex-encoded-signature>`

**Како се потпис рачуна:**
1. Конкатенишите: `timestamp + "." + JSON_payload_body`
2. Израчунајте HMAC-SHA256 користећи ваш API Secret као кључ
3. Хекс-енкодирајте резултат

### Примјер верификације (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Верификујте да је временска ознака недавна (у року од 5 минута)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Спречавање replay напада
    }

    // Верификујте потпис
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Примјер верификације (Python)

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

    # Верификујте да је временска ознака недавна
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Верификујте потпис
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Примјер верификације (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Верификујте да је временска ознака недавна (у року од 5 минута)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Верификујте потпис
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Насљедна аутентификација

Заглавље `token` које садржи ваш API Secret и даље се шаље ради повратне компатибилности. Међутим, препоручујемо прелазак на HMAC верификацију ради побољшане безбједности јер штити од поновљених напада.