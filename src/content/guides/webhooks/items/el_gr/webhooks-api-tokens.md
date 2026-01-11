---
Τα αιτήματα webhook του FastComments περιλαμβάνουν πολλούς μηχανισμούς πιστοποίησης για λόγους ασφάλειας.

## Κεφαλίδες που αποστέλλονται

| Κεφαλίδα | Περιγραφή |
|--------|-------------|
| `token` | Το API Secret σας (για συμβατότητα προς τα πίσω) |
| `X-FastComments-Timestamp` | Unix timestamp (δευτερόλεπτα) όταν υπογράφηκε το αίτημα |
| `X-FastComments-Signature` | HMAC-SHA256 υπογραφή του payload |

## Επαλήθευση Υπογραφής HMAC (Συνιστάται)

Συνιστούμε θερμά την επαλήθευση της υπογραφής HMAC για να διασφαλίσετε ότι τα payloads των webhook είναι αυθεντικά και δεν έχουν παραποιηθεί.

**Signature Format:** `sha256=<hex-encoded-signature>`

**Πώς υπολογίζεται η υπογραφή:**
1. Concatenate: `timestamp + "." + JSON_payload_body`
2. Compute HMAC-SHA256 using your API Secret as the key
3. Hex-encode the result

### Παράδειγμα Επαλήθευσης (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Επαλήθευση ότι το timestamp είναι πρόσφατο (εντός 5 λεπτών)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Πρόληψη επίθεσης επανάληψης
    }

    // Επαλήθευση υπογραφής
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Παράδειγμα Επαλήθευσης (Python)

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

    # Επαλήθευση ότι το timestamp είναι πρόσφατο
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Επαλήθευση υπογραφής
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Παράδειγμα Επαλήθευσης (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Επαλήθευση ότι το timestamp είναι πρόσφατο (εντός 5 λεπτών)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Επαλήθευση υπογραφής
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Παλιός Τρόπος Πιστοποίησης

Η κεφαλίδα `token` που περιέχει το API Secret σας εξακολουθεί να αποστέλλεται για συμβατότητα προς τα πίσω. Ωστόσο, συνιστούμε τη μετάβαση στην επαλήθευση HMAC για βελτιωμένη ασφάλεια καθώς προστατεύει από επιθέσεις επανάληψης.

---