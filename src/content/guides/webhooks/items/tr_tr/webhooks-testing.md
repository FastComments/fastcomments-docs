Yeni webhook ekleme ve düzenleme sayfalarında, formda o an bulunan URL'ye kaydedilmiş olup olmadığına bakılmaksızın istek gönderen bir `Send Test Payload` düğmesi bulunur. Create ve Update olayları sahte bir WebhookComment nesnesi gönderirken, Delete testi yalnızca bir ID içeren sahte bir istek gövdesi gönderir.

## Payload'ları Doğrulama

Webhook entegrasyonunuzu test ederken, gelen isteklerin aşağıdaki başlıkları içerdiğini doğrulayın:

1. **`X-FastComments-Timestamp`** - Unix zaman damgası (saniye)
2. **`X-FastComments-Signature`** - HMAC-SHA256 imzası

İmza şeması tanıtılmadan önce oluşturulmuş webhooks, API Secret'ınızı içeren bir **`token`** başlığı da alır. Yeni webhooks bunu almaz.

Payload'ların orijinal olduğundan emin olmak için HMAC imza doğrulamasını kullanın.

## Test Araçları

Geliştirme sırasında gelen webhook payload'larını incelemek için [webhook.site](https://webhook.site) veya [ngrok](https://ngrok.com) gibi araçları kullanabilirsiniz.

## Olay Türleri

- **Create Event**: Yeni bir yorum oluşturulduğunda tetiklenir.
- **Update Event**: Bir yorum düzenlendiğinde tetiklenir.
- **Delete Event**: Bir yorum silindiğinde tetiklenir.

Her webhook tek bir olaya ve tek bir HTTP metoduna (POST, PUT veya DELETE) bağlıdır. Her olay, istek gövdesinde tam yorum verisini içerir (payload formatı için [Veri Yapıları](/guide-webhooks.html#webhooks-structures) bölümüne bakın).