### Nimble Kullanımı

```bash
nimble install fastcomments
```

### Kaynak Koddan Derleme

```bash
nimble build
```

### Kütüphane İçeriği

Bu kütüphane, API ile çalışmayı kolaylaştırmak için oluşturulmuş API istemcisini ve SSO yardımcı araçlarını içerir.

- [API İstemci Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Genel ve Güvenli API'ler

API istemcisi için üç API modülü vardır: `api_default`, `api_public` ve `api_moderation`. `api_default`, API anahtarınızı gerektiren yöntemleri içerir ve `api_public`, tarayıcı/mobil cihaz vb. üzerinden kimlik doğrulama olmadan doğrudan yapılabilecek API çağrılarını içerir. `api_moderation` modülü ise moderatör panosu için yöntemleri içerir.

`api_moderation` yöntemleri yorumları ve bunların günlüklerini listeleme, sayma, arama ve dışa aktarma; yorumları kaldırma/geri yükleme, işaretleme, inceleme/spam/onay durumunu ayarlama, oyları ayarlama ve konuları yeniden açma/kapatma gibi moderasyon eylemlerini; yasaklar (bir kullanıcıyı bir yorumdan yasaklama, yasağı geri alma, ön-yasak özetleri, yasağın durumu ve tercihleri ve yasaklı kullanıcı sayıları); ve rozetler ve güven (rozet verme/kaldırma, manuel rozetleri listeleme, bir kullanıcının güven faktörünü alma/ayarlama ve bir kullanıcının dahili profilini alma) kapsar. Her `api_moderation` yöntemi bir `sso` parametresi kabul eder, böylece çağrı bir SSO moderatörü olarak kimlik doğrulanır.