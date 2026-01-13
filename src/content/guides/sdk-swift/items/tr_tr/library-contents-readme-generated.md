---
The FastComments Swift SDK birkaç modülden oluşur:

- **Client Modülü** - FastComments REST API'leri için otomatik oluşturulmuş API istemcisi
  - Tüm API modelleri için eksiksiz tip tanımları
  - Hem kimlik doğrulamalı (`DefaultAPI`) hem de genel (`PublicAPI`) uç noktalar
  - Tam async/await desteği
  - Ayrıntılı API dokümantasyonu için bkz. [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md)

- **SSO Modülü** - Sunucu tarafı Single Sign-On (SSO) araçları
  - Kullanıcı kimlik doğrulaması için güvenli token oluşturma
  - Hem basit hem de güvenli SSO modlarını destekler
  - CryptoKit kullanılarak HMAC-SHA256 tabanlı token imzalama
---