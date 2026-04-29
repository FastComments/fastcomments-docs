Her bir ajan webhook'unun kendi teslimat günlüğü vardır. Webhook satırındaki **Logs** düğmesi aracılığıyla [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) üzerinden erişilebilir.

### What's on the page

Sayfalı bir tablo; her teslimat denemesi için bir satır:

| Column | Meaning |
|---|---|
| When | When the attempt happened. |
| Event | The event type (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | The delivery status. |
| StatusCode | HTTP status code returned by your endpoint, when available. |
| Description | A short description of the outcome. For failed deliveries where no HTTP response was received, the underlying error message is stored as `{error: <message>}`. |

Sayfa yalnızca sayfalandırmayı destekler - durum, olay türü veya tarih aralığı filtreleri yoktur.

### What you can do from logs

- **Decide whether a status code should be in No-retry.** Eğer uç noktanızın aynı `4xx` hatayı defalarca döndürdüğünü görüyorsanız, platformun yeniden denemeyi durdurması için bunu **No-retry status codes** listesine eklemek istediğinize dair güçlü bir işarettir.

### Failure information

Bir teslimat HTTP yanıtı olmadan başarısız olduğunda (DNS hatası, bağlantı reddedildi, zaman aşımı, TLS hatası vb.), ham hata mesajı `{error: <message>}` olarak kaydedilir. Platform bunları `TIMEOUT` veya `DNS_ERROR` gibi adlandırılmış kategorilere ayırmaz — ne olduğunu görmek için hata mesajını doğrudan okuyun.

HTTP yanıtları için StatusCode sütunu uç noktanızın döndürdüğü kodu gösterir. Yaygın durumlar:

- **All attempts: `401` or `403`** - uç noktanız imzayı reddediyor. HMAC'i `${timestamp}.${body}` üzerinde hesapladığınızdan ve doğru sırrı kullandığınızdan emin olun. Bkz. [Webhook Signing](#webhook-signing).
- **All attempts: `422`** - uç noktanız yükü geçersiz olarak değerlendiriyor. Ya uç noktanızı düzeltin, ya da reddin belirli etkinlikler için beklenen bir durumsa `422`'yi **No-retry status codes** listesine ekleyin.

### Per-delivery context

Her günlük girdisi şunları taşır:

- `webhookId` - bu teslimatı üreten webhook yapılandırması.
- `agentId` - teslimatın ilgili olduğu ajan.
- `triggerId` or `approvalId` - ilgili kayıt.
- `domain` - eşleşen alan adı.

Bunları, başarısız bir teslimatı [Run History](#run-history) içindeki ilgili çalıştırma ile ilişkilendirmek için kullanabilirsiniz.

### Retention

`AgentSyncLog` girdilerinin `createdAt` üzerinde düz 1 yıllık bir saklama süresi (TTL) vardır; sonuç ne olursa olsun başarılı ve başarısız teslimatlar aynı süre boyunca saklanır. Saklama süresi sonrasında günlük girdisi silinir.

Uzun vadeli denetim gerekiyorsa sürdürülebilir desen şudur: teslimatları alan **endpoint'in kendisi** bu teslimatları saklasın. Bu, denetim günlüğünüzü platformun saklama politikasıyla decouple eder.

### Test send

Webhook yapılandırma formundaki **Test send** düğmesi, gerçek etkinliklere güvenmeden önce bağlantıyı uçtan uca doğrulayabilmeniz için aynı günlük tablosuna sahte bir teslimat yazar. Test teslimatları günlükte açıkça işaretlenir, böylece üretim hata istatistiklerini kirletmezler.

### See also

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) for the retry semantics that drive these logs.
- [Webhook Signing](#webhook-signing) for how to verify on your endpoint.