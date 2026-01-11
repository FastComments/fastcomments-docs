FastComments 的 webhook 請求包含多種驗證機制以確保安全性。

## Headers Sent

| Header | Description |
|--------|-------------|
| `token` | 您的 API Secret（為了向後相容） |
| `X-FastComments-Timestamp` | Unix 時間戳（秒），表示請求簽名的時間 |
| `X-FastComments-Signature` | 載荷的 HMAC-SHA256 簽名 |

## HMAC Signature Verification (Recommended)

我們強烈建議驗證 HMAC 簽名，以確保 webhook 載荷為真實且未被竄改。

**Signature Format:** `sha256=<hex-encoded-signature>`

**How the signature is computed:**
1. 串接： `timestamp + "." + JSON_payload_body`
2. 使用您的 API Secret 作為金鑰計算 HMAC-SHA256
3. 將結果進行十六進位編碼

### Example Verification (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // 驗證時間戳記是否為近期（5 分鐘內）
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // 防止重放攻擊
    }

    // 驗證簽名
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

    # 驗證時間戳記是否為近期
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # 驗證簽名
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

    // 驗證時間戳記是否為近期（5 分鐘內）
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // 驗證簽名
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Legacy Authentication

包含您 API Secret 的 `token` 標頭仍然會為了向後相容而發送。然而，我們建議遷移到 HMAC 驗證以提升安全性，因為它可以防止重放攻擊。

---