---
FastComments 웹훅 요청에는 보안을 위해 여러 인증 메커니즘이 포함되어 있습니다.

## 전송되는 헤더

| 헤더 | 설명 |
|--------|-------------|
| `token` | 귀하의 API Secret(하위 호환성용) |
| `X-FastComments-Timestamp` | 요청이 서명될 때의 Unix 타임스탬프(초) |
| `X-FastComments-Signature` | 페이로드의 HMAC-SHA256 서명 |

## HMAC 서명 검증 (권장)

웹훅 페이로드가 진본이며 변조되지 않았는지 확인하기 위해 HMAC 서명 검증을 강력히 권장합니다.

**서명 형식:** `sha256=<hex-encoded-signature>`

**서명이 계산되는 방법:**
1. 결합: `timestamp + "." + JSON_payload_body`
2. API Secret을 키로 사용하여 HMAC-SHA256을 계산
3. 결과를 16진수로 인코딩

### 예시 검증 (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // 타임스탬프가 최근인지 확인합니다 (5분 이내)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // 재생 공격 방지
    }

    // 서명 검증
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### 예시 검증 (Python)

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

    # 타임스탬프가 최신인지 확인합니다
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # 서명 검증
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### 예시 검증 (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // 타임스탬프가 최신인지 확인합니다 (5분 이내)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // 서명 검증
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## 레거시 인증

하위 호환성을 위해 귀하의 API Secret이 포함된 `token` 헤더는 여전히 전송됩니다. 그러나 HMAC 검증은 재생 공격으로부터 보호하므로 보안 향상을 위해 HMAC 검증으로 마이그레이션할 것을 권장합니다.

---