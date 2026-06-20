FastComments Swift SDK şu modüllerden oluşur:

- **İstemci Modülü** - FastComments REST API'leri için API istemcisi
  - Tüm API modelleri için eksiksiz tür tanımları
  - Yetkilendirilmiş (`DefaultAPI`), genel (`PublicAPI`) ve moderasyon (`ModerationAPI`) yöntemleri
  - Tam async/await desteği
  - Ayrıntılı API belgeleri için [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md)'e bakın

- **SSO Modülü** - Sunucu tarafı Tek Oturum Açma (SSO) yardımcıları
  - Kullanıcı kimlik doğrulaması için güvenli token oluşturma
  - Hem basit hem de güvenli SSO modlarını destekler
  - CryptoKit kullanarak HMAC-SHA256 tabanlı token imzalama