Agent webhook'leri başarısızlıkta yeniden deneme yapar. Teslimat, **agent açısından fire-and-forget (gönder ve unut)** şeklindedir — başarısız bir teslimat agent yürütmesini engellemez veya herhangi bir işlemi geri almaz — ve yeniden denemeleri asenkron olarak yöneten bir kuyruk + cron vardır.

### Kuyruk modeli

Her olay **eşleşen her webhook için bir kez** kuyruğa alınır. Yani belirli bir agent + domain için `trigger.succeeded` olayına abone üç webhook’unuz varsa, platform üç teslimat kuyruğa alır; her biri bağımsız olarak teslim edilir ve yeniden denenir. Bir webhook’taki hata diğerlerini asla etkilemez.

### Neler yeniden denenir

Bir teslimat şu durumlarda yeniden denenir:

- HTTP isteği **tamamlanmazsa** (DNS hatası, bağlantı reddedildi, zaman aşımı).
- HTTP yanıt kodu, yapılandırılmış **No-retry status codes** listesinde olmayan herhangi bir 2xx dışı statü kodu ise.

Bir teslimat **yeniden denenmez**:

- Yanıt kodu `2xx` ise (başarılı).
- Yanıt kodu yapılandırılmış **No-retry status codes** listesinde ise. Varsayılan olarak bu liste boştur — herhangi bir 2xx dışı kod yeniden denenir.

### No-retry kodlarını yapılandırma

Webhook konfigürasyon formunda **No-retry status codes** alanı (çoklu değer) vardır. Yaygın girdiler:

- `410` - Gone. Uç noktanız kalıcı olarak taşınmış veya kaynak kaybolmuş. Yeniden denemek her iki tarafın bant genişliğini boşa harcar.
- `422` - Unprocessable Entity. Uç noktanız yükü anladı ancak geçersiz saydı. Aynı yükle yeniden denemek aynı cevabı verir.
- `400` - Bad Request, aynı ruhla.

Buraya bir kod eklemek şu anlama gelir: uç nokta bunu döndürdüğünde, teslimatı failed-terminal olarak işaretle ve yeniden denemeyi durdur.

### Yeniden deneme takvimi

Arka plan çalışanı her birkaç saniyede bir çalışır ve bir sonraki deneme zamanı geçmiş olan teslimatları işler.

Her başarısızlıktan sonra, bir sonraki-deneme zamanı **doğrusal geri çekilme** ile ileri atılır: bekleme süresi `60 seconds * attempt count` olarak büyür (yani deneme 1 1 dakika bekler, deneme 2 2 dakika bekler, vb.).

99 başarısız denemeden sonra (yerel geliştirmede 3 denemede), teslimat vazgeçilmiş sayılır ve kuyruktan silinir. Teslimat günlük girişleri kalıcı olarak saklanır ve süresi dolana kadar [Webhook Delivery Logs](#webhook-logs) sayfasında görünür durumda kalır.

### Tarafınızda idempotence

Tekrar denediğimiz için, uç noktanızın **idempotent** olması gerekir. Aynı `triggerId` (veya `approvalId`) birden fazla gelebilir. Uç noktanız şunları yapmalıdır:

- Benzersiz bir anahtar (`triggerId` tetik olayları için, `approvalId` onay olayları için) kullanarak dedup token olarak kullanın.
- Yinelenen teslimatları nazikçe kabul edin (ikinci kez 200 döndürün).

Idempotent olmayan bir uç nokta, bazı teslimatları sonunda iki kez işleyecektir; özellikle bir zaman aşımı 30 saniye sonra yeniden denediğinde ancak orijinal istek aslında başarılı olduğunda bu durum görülür.

### Sıralama

Teslimatlar **kesinlikle sıralı değildir**. Aynı çalıştırmadan bir `trigger.succeeded` ile altındaki bir `approval.requested` (aynı çalıştırmadan) biri yeniden denerken diğeri denemezse herhangi bir sırayla gelebilir. Uç noktanız nedensel sıralama varsaymamalıdır.

Sıralamaya ihtiyacınız varsa, kendi tarafınızda sıralamayı yeniden oluşturmak için zaman damgalarını kullanın - zarf üzerindeki `occurredAt` ile veri bloğundaki tetik/onay `createdAt`.

### Temizlik

Teslimatlar ya başarılı olunca ya da deneme üst sınırına ulaşınca kuyruktan hemen kaldırılır. Platform terminal hatalı teslimatları kuyrukta saklamaz; her denemeye ait kalıcı kayıt [Webhook Delivery Logs](#webhook-logs) sayfasında bulunur.

### Yeniden denemeler başarısız olduğunda nerelere bakmalı

Bir webhook’un neden başarısız olduğunu görmek için [Webhook Delivery Logs](#webhook-logs) sayfasına bakın. Yaygın sebepler:

- **DNS çözümleme hatası** - URL yanlış veya domain kaybolmuş.
- **TLS hataları** - uç noktanızın sertifikası geçersiz veya süresi dolmuş.
- **Bağlantı reddedildi / zaman aşımı** - uç noktanız kapalı.
- **5xx yanıtları** - uç noktanız açık ama hata veriyor. Yanıt gövdesi (kısaltılmış) kaydedilir.
- **4xx yanıtları** - uç noktanız yükü reddetti. Eğer bu kasıtlıysa, kodu **No-retry status codes** listesine ekleyin.

### Sağlıksız bir webhook'u duraklatma

Bir webhook sürekli başarısız oluyorsa, en temiz çözüm onu silmek (veya geçici olarak olay abonelik listesini temizlemek) olur. Platform başarısız olan webhook’ları otomatik olarak devre dışı bırakmaz — teslimat vazgeçene kadar yeniden denemeye devam eder.