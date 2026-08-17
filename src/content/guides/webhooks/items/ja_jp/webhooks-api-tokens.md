FastComments の webhook リクエストは、セキュリティのために複数の認証メカニズムを含みます。

## 送信されるヘッダー

| ヘッダー | 説明 |
|--------|-------------|
| `token` | API シークレット (下位互換性のため) |
| `X-FastComments-Timestamp` | リクエストが署名されたときの Unix タイムスタンプ（秒） |
| `X-FastComments-Signature` | ペイロードの HMAC-SHA256 署名 |

## HMAC 署名検証（推奨）

Webhook のペイロードが正当で改ざんされていないことを保証するために、HMAC 署名の検証を強く推奨します。

**署名形式:** `sha256=<hex-encoded-signature>`

**署名の計算方法:**
1. 連結: `timestamp + "." + JSON_payload_body`
2. API シークレットをキーとして HMAC-SHA256 を計算
3. 結果を十六進エンコード

### 検証例 (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // タイムスタンプが最近（5 分以内）かを確認
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // リプレイ攻撃防止
    }

    // 署名を確認
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### 検証例 (Python)

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

    # タイムスタンプが最近かを確認
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # 署名を確認
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### 検証例 (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // タイムスタンプが最近（5 分以内）かを確認
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // 署名を確認
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## レガシー認証

`token` ヘッダーは下位互換性のために引き続き送信されますが、リプレイ攻撃から保護するために、セキュリティ向上の観点から HMAC 検証への移行を推奨します。