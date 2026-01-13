Sistemdeki Comment nesnesindeki tüm değişiklikler, bir kuyruğa düşen bir olay tetikler.

İlk webhook olayı genellikle olay kaynağının gerçekleşmesinden itibaren altı saniye içinde gönderilir.

API'niz çalışmayı durdurursa bu kuyruğu Webhooks yönetiminde izleyebilirsiniz.

API'nize yapılan bir istek başarısız olursa, onu belirli bir takvime göre yeniden kuyruğa alacağız.

That schedule is `1 Minute * the retry count`. If the call fails once, it'll try again in
bir dakika içinde. Eğer iki kez başarısız olursa, o zaman iki dakika bekleyecek ve böyle devam eder. Bu, böylece biz
API'nizi yükle ilgili nedenlerle kapanırken aşırı yüklememek içindir.

Webhooks, [kayıtlar sayfası](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs) üzerinden iptal edilebilir.