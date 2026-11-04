בקשות webhook של FastComments כוללות מספר מנגנוני אימות לצורך אבטחה.

## Headers שנשלחים

| Header | תיאור |
|--------|-------------|
| `token` | ה-API Secret שלך (לתאימות לאחור) |
| `X-FastComments-Timestamp` | חותמת זמן Unix (שניות) שבה הבקשה נחתמה |
| `X-FastComments-Signature` | חתימת HMAC-SHA256 של ה-payload |

## אימות חתימת HMAC (מומלץ)

אנו ממליצים בחום לאמת את חתימת ה-HMAC כדי לוודא ש-payloads של webhook הם אותנטיים ולא הושחתו.

**פורמט החתימה:** `sha256=<hex-encoded-signature>`

**כיצד החתימה מחושבת:**
1. שרשור: `timestamp + "." + JSON_payload_body`
2. חישוב HMAC-SHA256 תוך שימוש ב-API Secret שלך כמפתח
3. קידוד התוצאה ב-hex

### דוגמת אימות (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Verify timestamp is recent (within 5 minutes)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Replay attack prevention
    }

    // Verify signature
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

    # Verify timestamp is recent
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Verify signature
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

    // Verify timestamp is recent (within 5 minutes)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Verify signature
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## אימות מדור קודם

ה-header של `token` המכיל את ה-API Secret שלך עדיין נשלח לצורך תאימות לאחור. עם זאת, אנו ממליצים לעבור לאימות HMAC לשיפור האבטחה, מכיוון שהוא מגן מפני התקפות שידור חוזר (replay attacks).