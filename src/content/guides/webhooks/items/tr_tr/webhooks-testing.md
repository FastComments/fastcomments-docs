Webhooks yönetiminde her olay türü (Oluşturma, Güncelleme, Silme) için `Send Test Payload` düğmeleri vardır. Oluşturma ve Güncelleme olayları sahte bir `WebhookComment` nesnesi gönderir, Silme'yi test ederken ise yalnızca bir ID içeren sahte bir istek gövdesi gönderilir.

## Yükleri Doğrulama

Webhook entegrasyonunuzu test ederken, gelen isteklerin aşağıdaki başlıkları içerdiğini doğrulayın:

1. **`token`** - API Gizli Anahtarınız
2. **`X-FastComments-Timestamp`** - Unix zaman damgası (saniye)
3. **`X-FastComments-Signature`** - HMAC-SHA256 imzası

Yüklerin gerçekliğini sağlamak için HMAC imza doğrulamasını kullanın.

## Test Araçları

Geliştirme sırasında gelen webhook yüklerini incelemek için [webhook.site](https://webhook.site) veya [ngrok](https://ngrok.com) gibi araçları kullanabilirsiniz.

## Olay Türleri

- **Oluşturma Olayı**: Yeni bir yorum oluşturulduğunda tetiklenir. Varsayılan yöntem: PUT
- **Güncelleme Olayı**: Bir yorum düzenlendiğinde tetiklenir. Varsayılan yöntem: PUT
- **Silme Olayı**: Bir yorum silindiğinde tetiklenir. Varsayılan yöntem: DELETE

Her olay, istek gövdesinde tam yorum verisini içerir (yük formatı için bkz. [Veri Yapıları](/guide-webhooks.html#webhooks-structures)).