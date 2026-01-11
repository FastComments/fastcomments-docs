FastComments webhook захтеви укључују више механизама аутентификације ради безбедности.

## Заглавља која се шаљу

| Заглавље | Опис |
|--------|-------------|
| `token` | Ваш API Secret (ради уназадне компатибилности) |
| `X-FastComments-Timestamp` | Unix временска ознака (у секундама) када је захтев потписан |
| `X-FastComments-Signature` | HMAC-SHA256 потпис payload-а |

## Верификација HMAC потписа (Препоручено)

Снажно препоручујемо верификацију HMAC потписа како бисте били сигурни да су webhook payload-ови аутентични и да нису измењени.

**Формат потписа:** `sha256=<hex-encoded-signature>`

**Како се потпис израчунава:**
1. Конкатенирајте: `timestamp + "." + JSON_payload_body`
2. Израчунајте HMAC-SHA256 користећи ваш API Secret као кључ
3. Хекс-енкодирајте резултат

### Пример верификације (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Проверите да ли је временска ознака свежа (у року од 5 минута)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Превенција replay напада
    }

    // Проверите потпис
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

    # Проверите да ли је временска ознака свежа
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Проверите потпис
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

    // Проверите да ли је временска ознака свежа (у року од 5 минута)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Проверите потпис
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Наслеђена аутентификација

Заглавље `token` које садржи ваш API Secret и даље се шаље ради уназадне компатибилности. Међутим, препоручујемо прелазак на HMAC верификацију због побољшане безбедности јер штити од replay напада.