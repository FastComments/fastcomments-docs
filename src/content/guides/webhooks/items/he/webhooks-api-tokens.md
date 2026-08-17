FastComments webhook requests include multiple authentication mechanisms for security.

## Headers Sent

| כותרת | תיאור |
|--------|-------------|
| `token` | סוד ה-API שלך (לתאימות לאחור) |
| `X-FastComments-Timestamp` | חותמת זמן Unix (שניות) כאשר הבקשה נחתמה |
| `X-FastComments-Signature` | חתימת HMAC-SHA256 של המטען |

## HMAC Signature Verification (Recommended)

אנו ממליצים בחום לאמת את חתימת ה-HMAC כדי להבטיח שמטעני ה-webhook הם אותנטיים ולא שונו.

**פורמט החתימה:** `sha256=<hex-encoded-signature>`

**איך החתימה מחושבת:**
1. צירוף: `timestamp + "." + JSON_payload_body`
2. חישוב HMAC-SHA256 באמצעות סוד ה-API שלך כמפתח
3. קידוד תוצאה כ-hex

### דוגמת אימות (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // אימות שהחותמת זמן עדכנית (בתוך 5 דקות)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // מניעת התקפת השמעה
    }

    // אימות החתימה
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### דוגמת אימות (Python)

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

    # אימות שהחותמת זמן עדכנית
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # אימות החתימה
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### דוגמת אימות (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // אימות שהחותמת זמן עדכנית (בתוך 5 דקות)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // אימות החתימה
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Legacy Authentication

כותרת `token` המכילה את סוד ה-API שלך עדיין נשלחת לתאימות לאחור. עם זאת, אנו ממליצים לעבור לאימות HMAC לשיפור האבטחה מכיוון שהיא מגנה מפני התקפות השמעה.

---