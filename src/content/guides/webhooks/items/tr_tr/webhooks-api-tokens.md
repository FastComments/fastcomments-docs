FastComments webhook istekleri güvenlik için birden fazla kimlik doğrulama mekanizması içerir.

## Gönderilen Başlıklar

| Header | Açıklama |
|--------|-------------|
| `token` | API Gizli Anahtarınız (geriye dönük uyumluluk için) |
| `X-FastComments-Timestamp` | İsteğin imzalandığı Unix zaman damgası (saniye) |
| `X-FastComments-Signature` | Gönderinin HMAC-SHA256 imzası |

## HMAC İmza Doğrulaması (Önerilen)

Webhook gönderilerinin orijinal ve değiştirilmemiş olduğunu sağlamak için HMAC imzasını doğrulamanızı şiddetle tavsiye ederiz.

**İmza Formatı:** `sha256=<hex-encoded-signature>`

**İmzanın nasıl hesaplandığı:**
1. Birleştir: `timestamp + "." + JSON_payload_body`
2. Anahtar olarak API Gizli Anahtarınızı kullanarak HMAC-SHA256 hesaplayın
3. Sonucu hexadecimal (hex) olarak kodlayın

### Doğrulama Örneği (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhookSignature(req, apiSecret) {
    const timestamp = req.headers['x-fastcomments-timestamp'];
    const signature = req.headers['x-fastcomments-signature'];

    if (!timestamp || !signature) {
        return false;
    }

    // Zaman damgasının güncel olduğunu doğrula (5 dakika içinde)
    const now = Math.floor(Date.now() / 1000);
    if (Math.abs(now - parseInt(timestamp, 10)) > 300) {
        return false;  // Tekrar oynatma (replay) saldırılarını önlemek için
    }

    // İmzayı doğrula
    const payload = JSON.stringify(req.body);
    const expectedSignature = crypto
        .createHmac('sha256', apiSecret)
        .update(`${timestamp}.${payload}`)
        .digest('hex');

    return signature === `sha256=${expectedSignature}`;
}
```

### Doğrulama Örneği (Python)

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

    # Zaman damgasının güncel olduğunu doğrula
    now = int(time.time())
    if abs(now - int(timestamp)) > 300:
        return False

    # İmzayı doğrula
    payload = json.dumps(body, separators=(',', ':'))
    message = f"{timestamp}.{payload}"
    expected = hmac.new(
        api_secret.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return signature == f"sha256={expected}"
```

### Doğrulama Örneği (PHP)

```php
function verifyWebhookSignature($headers, $body, $apiSecret) {
    $timestamp = $headers['X-FastComments-Timestamp'] ?? null;
    $signature = $headers['X-FastComments-Signature'] ?? null;

    if (!$timestamp || !$signature) {
        return false;
    }

    // Zaman damgasının güncel olduğunu doğrula (5 dakika içinde)
    $now = time();
    if (abs($now - intval($timestamp)) > 300) {
        return false;
    }

    // İmzayı doğrula
    $payload = json_encode($body, JSON_UNESCAPED_SLASHES);
    $message = $timestamp . '.' . $payload;
    $expectedSignature = 'sha256=' . hash_hmac('sha256', $message, $apiSecret);

    return hash_equals($expectedSignature, $signature);
}
```

## Eski Kimlik Doğrulama

`token` başlığı, API Gizli Anahtarınızı içeren, geriye dönük uyumluluk için hâlâ gönderilmektedir. Ancak, tekrar oynatma saldırılarına karşı koruma sağladığı için geliştirilmiş güvenlik için HMAC doğrulamasına geçmenizi öneririz.

---