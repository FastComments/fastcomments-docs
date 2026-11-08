FastComments webhook अनुरोधों में सुरक्षा के लिए कई प्रमाणीकरण तंत्र शामिल हैं।

## भेजे गए हेडर

| हेडर | विवरण |
|--------|-------------|
| `token` | आपका API Secret (पिछली संगतता के लिए) |
| `X-FastComments-Timestamp` | Unix टाइमस्टैम्प (सेकंड) जब अनुरोध पर हस्ताक्षर किया गया |
| `X-FastComments-Signature` | पेलोड का HMAC-SHA256 हस्ताक्षर |

## HMAC हस्ताक्षर सत्यापन (अनुशंसित)

हम दृढ़ता से अनुशंसा करते हैं कि HMAC हस्ताक्षर को सत्यापित किया जाए ताकि यह सुनिश्चित हो सके कि webhook पेलोड प्रामाणिक हैं और उनमें छेड़छाड़ नहीं की गई है।

**हस्ताक्षर प्रारूप:** `sha256=<hex-encoded-signature>`

**हस्ताक्षर की गणना कैसे होती है:**
1. जोड़ें: `timestamp + "." + JSON_payload_body`
2. अपने API Secret को कुंजी के रूप में उपयोग करके HMAC-SHA256 गणना करें
3. परिणाम को हेक्स-एनकोड करें

### सत्यापन उदाहरण (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // सत्यापित करें कि टाइमस्टैम्प हाल का है (5 मिनट के भीतर)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // रीप्ले अटैक की रोकथाम
    }

    // हस्ताक्षर सत्यापित करें
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### सत्यापन उदाहरण (Python)

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

    # सत्यापित करें कि टाइमस्टैम्प हाल का है
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # हस्ताक्षर सत्यापित करें
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### सत्यापन उदाहरण (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // सत्यापित करें कि टाइमस्टैम्प हाल का है (5 मिनट के भीतर)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // हस्ताक्षर सत्यापित करें
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## लेगेसी प्रमाणीकरण

आपके API Secret युक्त `token` हेडर पिछली संगतता के लिए अभी भी भेजा जाता है। हालाँकि, बेहतर सुरक्षा के लिए हम HMAC सत्यापन पर माइग्रेट करने की अनुशंसा करते हैं क्योंकि यह रीप्ले अटैक से सुरक्षा प्रदान करता है।