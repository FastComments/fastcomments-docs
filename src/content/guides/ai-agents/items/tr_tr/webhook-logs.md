Her agent webhook'unun kendi teslimat günlüğü vardır. [webhook liste sayfasından](https://fastcomments.com/auth/my-account/ai-agents/webhooks) her webhook satırındaki **Logs** düğmesi üzerinden erişilebilir.

### Sayfada neler var

Her teslimat denemesi için bir satır içeren sayfalandırılmış bir tablo:

| Sütun | Anlamı |
|---|---|
| When | Denemenin gerçekleştiği zaman. |
| Event | Olay türü (`trigger.succeeded`, `approval.requested` vb.). |
| Status | Teslimat durumu. |
| StatusCode | Uç noktanızın döndürdüğü HTTP durum kodu (varsa). |
| Description | Sonucun kısa açıklaması. HTTP yanıtı alınmayan başarısız teslimatlarda, altta yatan hata mesajı `{error: <message>}` olarak saklanır. |

Sayfa yalnızca sayfalandırmayı destekler - durum, olay türü veya tarih aralığı filtresi yoktur.

### Günlüklerden neler yapabilirsiniz

- **Bir durum kodunun No-retry'da olup olmayacağına karar verin.** Uç noktanızın aynı `4xx` kodunu tekrar tekrar döndürdüğünü görüyorsanız, bu, platformun yeniden denemeyi durdurması için onu **No-retry status codes** listesine eklemek istediğinizin güçlü bir işaretidir.

### Hata bilgileri

Bir teslimat HTTP yanıtı olmadan başarısız olduğunda (DNS hatası, bağlantı reddedildi, zaman aşımı, TLS hatası vb.), ham hata mesajı `{error: <message>}` olarak kaydedilir. Platform bunları `TIMEOUT` veya `DNS_ERROR` gibi adlandırılmış kategorilere ayırmaz - ne olduğunu görmek için hata mesajını doğrudan okuyun.

HTTP yanıtları için StatusCode sütunu, uç noktanızın döndürdüğü kodu gösterir. Yaygın durumlar:

- **Tüm denemeler: `401` veya `403`** - uç noktanız imzayı reddediyor. HMAC'i `${timestamp}.${body}` üzerinde hesapladığınızdan ve doğru gizli anahtarı kullandığınızdan emin olun. Bkz. [Webhook Signing](#webhook-signing).
- **Tüm denemeler: `422`** - uç noktanız yükün geçersiz olduğunu düşünüyor. Ya uç noktanızı düzeltin ya da reddetme bazı olaylar için bekleniyorsa `422` kodunu **No-retry status codes** listesine ekleyin.

### Teslimat başına bağlam

Her günlük girdisi şunları taşır:

- `webhookId` - bu teslimatı üreten webhook yapılandırması.
- `agentId` - teslimatın ilgili olduğu agent.
- `triggerId` veya `approvalId` - altta yatan kayıt.
- `domain` - eşleşen alan adı.

Başarısız bir teslimatı, [Run History](#run-history) bölümündeki ilgili çalışmayla ilişkilendirmek için bunları kullanabilirsiniz.

### Saklama

`AgentSyncLog` girdileri, sonuçtan bağımsız olarak `createdAt` üzerinde düz bir 1 yıllık TTL'ye sahiptir - başarılı ve başarısız teslimatlar aynı süre boyunca saklanır. Saklama süresinin ötesinde, günlük girdisi silinir.

Uzun vadeli denetime ihtiyacınız varsa, sürdürülebilir kalıp şudur: **uç noktanın kendisi** aldığı teslimatları kalıcı olarak saklasın. Bu, denetim günlüğünüzü platformun saklama politikasından ayırır.

### Test gönderimi

Webhook yapılandırma formundaki **Test send** düğmesi, gerçek olaylara güvenmeden önce bağlantıyı uçtan uca doğrulayabilmeniz için aynı günlük tablosuna sahte bir teslimat yazar. Test teslimatları, üretim hata istatistiklerini kirletmemeleri için günlükte açıkça etiketlenir.

### Ayrıca bkz.

- [Webhooks Overview](#webhooks-overview).
- Bu günlükleri yönlendiren yeniden deneme semantiği için [Webhook Retries](#webhook-retries).
- Uç noktanızda nasıl doğrulama yapılacağı için [Webhook Signing](#webhook-signing).