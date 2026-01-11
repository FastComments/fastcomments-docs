FastComments webhook requests include multiple authentication mechanisms for security.

## Изпращани заглавки

| Header | Description |
|--------|-------------|
| `token` | Вашият API Secret (за обратна съвместимост) |
| `X-FastComments-Timestamp` | Unix времеви печат (в секунди), когато заявката е била подписана |
| `X-FastComments-Signature` | HMAC-SHA256 подпис на полезното съдържание |

## HMAC Signature Verification (Recommended)

Силно препоръчваме да проверявате HMAC подписа, за да се уверите, че payload-ите на webhook-ите са автентични и не са били подправяни.

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

    // Проверете дали времевият печат е скорошен (в рамките на 5 минути)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Предотвратяване на replay атаки
    }

    // Проверете подписа
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

    # Проверете дали времевият печат е скорошен
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Проверете подписа
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

    // Проверете дали времевият печат е скорошен (в рамките на 5 минути)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Проверете подписа
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Наследствено удостоверяване

Хедърът `token`, съдържащ вашия API Secret, все още се изпраща за обратна съвместимост. Въпреки това препоръчваме миграция към проверка на HMAC за по-добра сигурност, тъй като тя предпазва от replay атаки.