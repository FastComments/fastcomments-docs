FastComments Rust SDK'sı birkaç modülden oluşur:

- **Client Module** - FastComments REST API'leri için API istemcisi
  - Tüm API modelleri için tam tip tanımları
  - Tüm FastComments yöntemlerini kapsayan üç API istemcisi:
    - `default_api` (**DefaultApi**) - Sunucu tarafı kullanım için API anahtarı ile kimlik doğrulamalı yöntemler
    - `public_api` (**PublicApi**) - Tarayıcılar ve mobil uygulamalardan güvenli bir şekilde çağrılabilen, API anahtarı gerektirmeyen genel yöntemler
    - `moderation_api` (**ModerationApi**) - Canlı ve hızlı denetleme API'lerinin kapsamlı bir paketi. Her Denetleme yöntemi bir `sso` parametresi alır ve SSO veya bir FastComments.com oturum çerezi ile kimlik doğrulaması yapabilir.
  - tokio ile tam async/await desteği
  - Ayrıntılı API belgeleri için [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) sayfasına bakın

- **SSO Module** - Sunucu tarafı Tek Oturum Açma (Single Sign-On) yardımcı programları
  - Kullanıcı kimlik doğrulaması için güvenli token oluşturma
  - Hem basit hem de güvenli SSO modları için destek
  - HMAC-SHA256 tabanlı token imzalama

- **Core Types** - Paylaşılan tip tanımları ve yardımcı programlar
  - Yorum modelleri ve meta veri yapıları
  - Kullanıcı ve kiracı (tenant) yapılandırmaları
  - Yaygın işlemler için yardımcı işlevler