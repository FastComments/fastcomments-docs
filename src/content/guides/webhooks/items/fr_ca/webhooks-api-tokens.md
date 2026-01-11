FastComments webhook requests include multiple authentication mechanisms for security.

## En-têtes envoyés

| En-tête | Description |
|--------|-------------|
| `token` | Votre API Secret (pour rétrocompatibilité) |
| `X-FastComments-Timestamp` | Timestamp Unix (secondes) indiquant quand la requête a été signée |
| `X-FastComments-Signature` | Signature HMAC-SHA256 de la charge utile |

## Vérification de la signature HMAC (recommandée)

Nous recommandons fortement de vérifier la signature HMAC afin de vous assurer que les charges utiles des webhooks sont authentiques et n'ont pas été modifiées.

**Format de la signature :** `sha256=<hex-encoded-signature>`

**Comment la signature est calculée :**
1. Concaténer : `timestamp + "." + JSON_payload_body`
2. Calculer HMAC-SHA256 en utilisant votre API Secret comme clé
3. Encoder le résultat en hexadécimal

### Exemple de vérification (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Vérifier que le timestamp est récent (moins de 5 minutes)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Prévention des attaques par rejeu
    }

    // Vérifier la signature
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Exemple de vérification (Python)

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

    # Vérifier que le timestamp est récent
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # Vérifier la signature
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Exemple de vérification (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Vérifier que le timestamp est récent (moins de 5 minutes)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // Vérifier la signature
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Authentification héritée

L'en-tête `token` contenant votre API Secret est toujours envoyé pour des raisons de rétrocompatibilité. Toutefois, nous recommandons de migrer vers la vérification HMAC pour une meilleure sécurité, car elle protège contre les attaques par rejeu.