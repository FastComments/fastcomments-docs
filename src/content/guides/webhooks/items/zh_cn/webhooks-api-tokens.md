FastComments 的 webhook 请求包含多种身份验证机制以确保安全。

## 发送的请求头

| 请求头 | 描述 |
|--------|-------------|
| `token` | 您的 API Secret（用于向后兼容） |
| `X-FastComments-Timestamp` | 请求签名时的 Unix 时间戳（秒） |
| `X-FastComments-Signature` | 负载的 HMAC-SHA256 签名 |

## HMAC 签名验证（推荐）

我们强烈建议验证 HMAC 签名，以确保 webhook payload 是真实且未被篡改。

**Signature Format:** `sha256=<hex-encoded-signature>`

**签名的计算方法：**
1. 连接： `timestamp + "." + JSON_payload_body`
2. 使用您的 API Secret 作为密钥计算 HMAC-SHA256
3. 对结果进行十六进制编码

### 示例验证（Node.js）

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // 验证时间戳是否近期（在 5 分钟内）
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // 防止重放攻击
    }

    // 验证签名
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### 示例验证（Python）

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

    # 验证时间戳是否近期
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # 验证签名
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### 示例验证（PHP）

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // 验证时间戳是否近期（在 5 分钟内）
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // 验证签名
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## 旧版身份验证

包含您 API Secret 的 `token` 请求头仍会出于向后兼容而发送。然而，我们建议迁移到 HMAC 验证以提高安全性，因为它可以防止重放攻击。