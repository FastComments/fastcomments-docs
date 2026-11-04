FastComments 的 webhook 請求包含多種驗證機制以確保安全。

## 傳送的標頭

| 標頭 | 說明 |
|--------|-------------|
| `token` | 您的 API Secret(為了向下相容) |
| `X-FastComments-Timestamp` | 請求簽署時的 Unix 時間戳(秒) |
| `X-FastComments-Signature` | 負載的 HMAC-SHA256 簽名 |

## HMAC 簽名驗證(建議)

我們強烈建議驗證 HMAC 簽名,以確保 webhook 負載是真實的且未被竄改。

**簽名格式:** `sha256=<hex-encoded-signature>`

**簽名的計算方式:**
1. 串接:`timestamp + "." + JSON_payload_body`
2. 以您的 API Secret 作為金鑰計算 HMAC-SHA256
3. 將結果以十六進位編碼

### 驗證範例(Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // 驗證時間戳是否為近期(5 分鐘內)
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

### 驗證範例(Python)

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

    # 驗證時間戳是否為近期
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

### 驗證範例(PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // 驗證時間戳是否為近期(5 分鐘內)
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

## 舊版驗證方式

包含您 API Secret 的 `token` 標頭仍會傳送以維持向下相容。然而,我們建議遷移至 HMAC 驗證以提升安全性,因為它可以防止重放攻擊。