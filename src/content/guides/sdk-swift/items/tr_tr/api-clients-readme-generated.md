FastComments SDK üç API istemcisi sağlar:

### PublicAPI - İstemci-Güvenli Yöntemler

`PublicAPI`, istemci tarafı kodundan (iOS/macOS uygulamaları) çağırılması güvenli olan yöntemler içerir. Bu yöntemler:
- API anahtarı gerektirmez
- Kimlik doğrulama için SSO belirteçlerini kullanabilir
- Kullanıcı/cihaz başına oran sınırlamalıdır
- Son kullanıcıya yönelik uygulamalar için uygundur

**Örnek kullanım senaryosu**: iOS uygulamanızda yorumları getirme ve oluşturma

### DefaultAPI - Sunucu-Tarafı Yöntemler

`DefaultAPI`, bir API anahtarı gerektiren kimliği doğrulanmış yöntemler içerir. Bu yöntemler:
- FastComments API anahtarınızı gerektirir
- SADECE sunucu tarafı kodundan çağrılmalıdır
- FastComments verilerinize tam erişim sağlar
- Kiracı başına oran sınırlaması vardır

**Örnek kullanım senaryosu**: Yönetim operasyonları, toplu veri dışa aktarımı, kullanıcı yönetimi

### ModerationAPI - Moderatör Kontrol Paneli Yöntemleri

`ModerationAPI`, canlı ve hızlı moderasyon API'lerinin kapsamlı bir paketini sunar. Her `ModerationAPI` yöntemi bir `sso` parametresi alır ve SSO veya bir FastComments.com oturum çerezi ile kimlik doğrulaması yapabilir.

**Örnek kullanım senaryosu**: Topluluğunuzdaki moderatörler için bir moderasyon deneyimi oluşturma

**ÖNEMLİ**: API anahtarınızı istemci tarafı kodda asla açığa çıkarmayın. API anahtarları yalnızca sunucu tarafında kullanılmalıdır.