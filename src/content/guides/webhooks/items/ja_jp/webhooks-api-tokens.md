---
FastComments webhook requests include multiple authentication mechanisms for security.

## Headers Sent

| Header | Description |
|--------|-------------|
| `token` | Your API Secret (for backwards compatibility) |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature of the payload |

## HMAC Signature Verification (Recommended)

We strongly recommend verifying the HMAC signature to ensure webhook payloads are authentic and haven't been tampered with.

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

    // タイムスタンプが最近のものであることを検証する（5分以内）
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // リプレイ攻撃の防止
    }

    // 署名を検証する
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

    # タイムスタンプが最近のものであることを検証する
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # 署名を検証する
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

    // タイムスタンプが最近のものであることを検証する（5分以内）
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // 署名を検証する
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Legacy Authentication

The `token` header containing your API Secret is still sent for backwards compatibility. However, we recommend migrating to HMAC verification for improved security as it protects against replay attacks.

---