Her ajan webhook'u kiracınızın API gizli anahtarı kullanılarak HMAC-SHA256 ile imzalanır. Aynı imzalama şeması FastComments'ın yorum webhook'ları için de kullanılır - eğer bunları zaten entegre ettiyseniz, ajan webhook'ları aynı imza başlığını ve doğrulama akışını yeniden kullanır.

### İmzalamanın nedeni

İmza olmadan, webhook URL'nizi bilen bir saldırgan FastComments'tan geliyormuş gibi görünen sahte POST istekleri gönderebilir. İmzalama, uç noktanızın her teslimatın gerçek olduğundan emin olup ona göre işlem yapmasını sağlar.

### İmzalar nasıl çalışır

Her teslimat için:

1. Platform, kiracı + eşleşen alan adı için API gizli anahtarını arar (bkz. [Webhooks Overview](#webhooks-overview)).
2. `X-FastComments-Timestamp` başlığında geçerli Unix zaman damgasını (milisaniye cinsinden) yayımlar.
3. `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (Stripe tarzı) hesaplar ve sonucu `X-FastComments-Signature` başlığında `sha256=<hex>` olarak yayımlar.
4. Uç noktanız zaman damgası başlığını okur, aldığı `${timestamp}.${body}` üzerinde HMAC'i yeniden hesaplar, imza başlığındaki `sha256=<hex>` değeri ile karşılaştırır ve uyuşmazlıkları reddeder.

İmzalanan gövde, platformun gönderdiği **tam baytlar** olup `${timestamp}.` ile ön eklenir - doğrulayıcınız ham istek gövdesini kullanmalıdır, yeniden serileştirilmiş bir JSON dizesini değil (anahtar sıralaması ve boşluk farklılık gösterebilir).

### API gizli anahtarı

Aynı API Secret, [comment webhooks](/guide-webhooks.html) tarafından kullanılır. Bu değer (kiracı, alan adı) başına olup kiracınızın API ayarlarında yönetilir. Gizli anahtarı döndürdüğünüzde, bir sonraki teslimattan önce yeni değeri okuyacak şekilde doğrulayıcınızı yeniden dağıtmalısınız.

Platform, eşleşen alan adı için **hiç API secret bulamazsa**, teslimat gerçekleşmez. Webhook günlüğü sebep olarak "no API secret" hatası ile başarısızlığı kaydeder.

### Doğrulama örneği (Node.js)

[inline-code-attrs-start title = 'Webhook İmza Doğrulama Örneği'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

İmzanın zamanlama kanalı sızıntılarını önlemek için `===` yerine `timingSafeEqual` kullanın.

### İmzalanmış gövdede ne var

Tam zarf (envelope) artı olaya özgü `data` bloğu. Bkz. [Webhook Payloads](#webhook-payloads).

### Öneriler

- **Her teslimatta doğrulayın.** Uç noktanız imzasız istekleri kabul ediyorsa, bütünlük garantisi yoktur.
- **İmza uyuşmazlığında reddedin.** 401 veya 403 döndürün; kötü bir imzada 200 OK dönmeyin, aksi takdirde teslimat günlüklerinizdeki saldırıları maskelersiniz.
- **HTTPS kullanın.** İmzalar bütünlüğü korur; TLS gizliliği korur (hem gizli anahtarınız hem de yük içindeki yorum metni).
- **Gizli anahtarları döndürün** erişimi olan ekip üyeleri ayrıldığında veya belirli bir takvime göre.

### Yeniden oynatma koruması

Sadece imzalama yeniden oynatma (replay) saldırılarını önlemez - gerçek imzalı bir teslimatı yakalayan bir saldırgan bunu yeniden gönderebilir. Yeniden oynatma koruması sizin uç noktanıza bağlıdır:

- Zarf alanı `occurredAt`'i kullanın ve örneğin 5 dakikadan daha eski teslimatları reddedin.
- `triggerId` veya `approvalId`'yi dedup anahtarı olarak kullanın - zaten işlediyseniz, çoğaltmayı yoksayınız.

### Ayrıca bakınız

- [Webhooks Overview](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Daha geniş imzalama altyapısı için ana [Webhooks guide](/guide-webhooks.html).