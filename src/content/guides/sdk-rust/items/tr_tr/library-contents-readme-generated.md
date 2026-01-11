---
The FastComments Rust SDK birkaç modülden oluşur:

- **İstemci Modülü** - FastComments REST API'leri için otomatik oluşturulmuş API istemcisi
  - Tüm API modelleri için eksiksiz tür tanımları
  - Hem kimlik doğrulamalı (`DefaultApi`) hem de genel (`PublicApi`) uç noktalar
  - tokio ile tam async/await desteği
  - Detaylı API belgelendirmesi için [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) bakınız

- **SSO Modülü** - Sunucu tarafı Tek Oturum Açma (Single Sign-On) araçları
  - Kullanıcı kimlik doğrulaması için güvenli token oluşturma
  - Hem basit hem de güvenli SSO modları için destek
  - HMAC-SHA256 tabanlı token imzalama

- **Temel Türler** - Paylaşılan tür tanımları ve yardımcı araçlar
  - Yorum modelleri ve meta veri yapıları
  - Kullanıcı ve kiracı yapılandırmaları
  - Yaygın işlemler için yardımcı fonksiyonlar
---