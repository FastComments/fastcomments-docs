---
Τα αιτήματα webhook του FastComments περιλαμβάνουν πολλαπλούς μηχανισμούς αυθεντικοποίησης για λόγους ασφάλειας.

## Αποστελλόμενες κεφαλίδες

| Κεφαλίδα | Περιγραφή |
|--------|-------------|
| `token` | Το API Secret σας (για συμβατότητα προς τα πίσω) |
| `X-FastComments-Timestamp` | Unix χρονοσφραγίδα (σε δευτερόλεπτα) όταν το αίτημα υπογράφηκε |
| `X-FastComments-Signature` | HMAC-SHA256 υπογραφή του payload |

## Επαλήθευση HMAC Υπογραφής (Συνιστάται)

Συνιστούμε έντονα να επαληθεύετε την HMAC υπογραφή για να διασφαλίσετε ότι τα payload των webhooks είναι αυθεντικά και δεν έχουν παραποιηθεί.

**Μορφή Υπογραφής:** `sha256=<hex-encoded-signature>`

**Πώς υπολογίζεται η υπογραφή:**
1. Συνένωση: `timestamp + "." + JSON_payload_body`
2. Υπολογισμός HMAC-SHA256 χρησιμοποιώντας το API Secret σας ως κλειδί
3. Κωδικοποιήστε το αποτέλεσμα σε hex

### Παράδειγμα Επαλήθευσης (Node.js)

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

### Παράδειγμα Επαλήθευσης (PHP)

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

## Παρωχημένη Αυθεντικοποίηση

Η κεφαλίδα `token` που περιέχει το API Secret σας εξακολουθεί να αποστέλλεται για συμβατότητα με παλαιότερες εκδόσεις. Ωστόσο, προτείνουμε τη μετάβαση στην επαλήθευση HMAC για βελτιωμένη ασφάλεια καθώς προστατεύει από επιθέσεις επανάληψης.

---