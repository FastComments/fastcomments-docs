Webhooks yönetim panelinde her bir etkinlik türü (Create, Update, Delete) için `Send Test Payload` düğmeleri bulunur. Create ve Update etkinlikleri sahte bir WebhookComment object gönderirken, Delete testi yalnızca bir ID içeren sahte bir istek gövdesi gönderecektir.

## Verifying Payloads

Webhook entegrasyonunuzu test ederken, gelen isteklerin aşağıdaki başlıkları içerdiğini doğrulayın:

1. **`token`** - API Gizli Anahtarınız
2. **`X-FastComments-Timestamp`** - Unix zaman damgası (saniye)
3. **`X-FastComments-Signature`** - HMAC-SHA256 imzası

Gelen payload'ların gerçekliğini sağlamak için HMAC imza doğrulamasını kullanın.

## Testing Tools

Geliştirme sırasında gelen webhook payload'larını incelemek için [webhook.site](https://webhook.site) veya [ngrok](https://ngrok.com) gibi araçları kullanabilirsiniz.

## Event Types

- **Create Event**: Yeni bir yorum oluşturulduğunda tetiklenir. Varsayılan yöntem: PUT
- **Update Event**: Bir yorum düzenlendiğinde tetiklenir. Varsayılan yöntem: PUT
- **Delete Event**: Bir yorum silindiğinde tetiklenir. Varsayılan yöntem: DELETE

Her etkinlik, istek gövdesinde tam yorum verilerini içerir (yük formatı için bkz. [Veri Yapıları](/guides/webhooks/webhooks-structures)).