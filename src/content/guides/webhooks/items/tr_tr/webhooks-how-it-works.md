Sistemdeki Comment nesnesine yapılan tüm değişiklikler, bir kuyruğa düşen bir olay tetikler.

İlk webhook olayı genellikle olay kaynağı gerçekleştikten sonraki altı saniye içinde gönderilir.

API'niz devre dışı kalması durumunda, bu kuyruğu Webhooks yönetim panelinden izleyebilirsiniz.

API'nize yapılan bir istek başarısız olursa, bunu bir zamanlamaya göre yeniden kuyruğa alırız.

Bu zamanlama `1 Dakika * yeniden deneme sayısı` şeklindedir. Çağrı bir kez başarısız olursa, bir dakika sonra tekrar dener. İki kez başarısız olursa, iki dakika bekler ve bu böyle devam eder. Bu, yük kaynaklı nedenlerle düşüyorsanız API'nizi aşırı yüklememek içindir.

Webhook'lar [logs sayfasından](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs) iptal edilebilir.