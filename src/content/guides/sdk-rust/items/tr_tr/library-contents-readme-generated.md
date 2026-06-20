The FastComments Rust SDK birkaç modülden oluşur:

- **Client Module** - FastComments REST API'leri için API istemcisi
  - Tüm API modelleri için eksiksiz tip tanımları
  - Tüm FastComments yöntemlerini kapsayan üç API istemcisi:
    - `default_api` (**DefaultApi**) - sunucu tarafı kullanım için API anahtarıyla doğrulanan yöntemler
    - `public_api` (**PublicApi**) - tarayıcılardan ve mobil uygulamalardan güvenle çağrılabilecek, API anahtarı gerektirmeyen genel yöntemler
    - `moderation_api` (**ModerationApi**) - moderatör panosunu destekleyen yöntemler; yorum moderasyonu (listeleme, sayma, arama, günlükler, dışa aktarma), moderasyon işlemleri (kaldır/geri al, işaretle, inceleme/spam/onay durumunu ayarla, oylar, konu yeniden aç/kapat), yasaklar (bir yorumdan yasaklama, geri alma, ön-yasak özetleri, yasak durumu/tercihleri, yasaklı kullanıcı sayıları) ve rozetler & güven (rozet verme/kaldırma, manuel rozetler, güven faktörünü alma/ayarlama, kullanıcı iç profili). Her Moderation yöntemi bir `sso` parametresi kabul eder, böylece çağrı SSO ile kimlik doğrulanmış bir moderatör adına yapılabilir.
  - tokio ile tam async/await desteği
  - Ayrıntılı API dökümantasyonu için bkz. [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md)

- **SSO Module** - Sunucu tarafı Single Sign-On yardımcıları
  - Kullanıcı kimlik doğrulaması için güvenli token oluşturma
  - Basit ve güvenli SSO modlarını destekleme
  - HMAC-SHA256 tabanlı token imzalama

- **Core Types** - Paylaşılan tip tanımları ve yardımcılar
  - Yorum modelleri ve meta veri yapıları
  - Kullanıcı ve tenant yapılandırmaları
  - Yaygın işlemler için yardımcı fonksiyonlar