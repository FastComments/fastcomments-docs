### Bağımlılıkları Yükleme

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Kaynaktan Derleme

```bash
mkdir build
cd build
cmake ..
make
```

### Kurulum

```bash
sudo make install
```

### Kütüphane İçeriği

Bu kütüphane, oluşturulmuş API istemcisini ve API ile çalışmayı kolaylaştıran SSO yardımcılarını içerir.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Genel vs Güvenli API'ler

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerir ve `PublicApi` kimlik doğrulama olmadan doğrudan bir tarayıcı/mobil cihaz/vb üzerinden yapılabilen yöntemleri içerir. `ModerationApi`, moderatör panosunu işleten yöntemleri içerir — yorumlar için listeleme, sayma, arama, dışa aktarma ve günlük çekme; moderasyon işlemleri (kaldır/geri yükle, işaretleme, inceleme/spam/onay durumunu ayarlama, oyları ayarlama, konu başlıklarını yeniden aç/kapat); yasaklar (bir yorumdan yasaklama, yasakları geri alma, ön-yasak özetleri, yasak durumu ve tercihleri, yasaklı kullanıcı sayıları); ve rozetler & güven (rozet verme/kaldırma, manuel rozetler, güven faktörünü alma/ayarlama, kullanıcı iç profili). Her `ModerationApi` yöntemi bir `sso` parametresi kabul eder, böylece çağrı SSO ile kimlik doğrulanmış bir moderatör adına gerçekleştirilir.