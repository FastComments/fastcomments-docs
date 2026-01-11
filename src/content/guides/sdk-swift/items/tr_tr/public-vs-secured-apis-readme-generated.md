FastComments SDK, iki tür API uç noktası sağlar:

### `PublicAPI` - İstemci tarafı için güvenli uç noktalar

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS apps). These endpoints:
- Bir API key gerektirmez
- Kimlik doğrulama için SSO tokens kullanabilir
- Her kullanıcı/cihaz için istek sınırlaması vardır
- Son kullanıcıya yönelik uygulamalar için uygundur

**Örnek kullanım**: iOS uygulamanızda yorumları getirme ve oluşturma

### `DefaultAPI` - Sunucu tarafı uç noktalar

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- FastComments API key'inizi gerektirir
- SADECE sunucu tarafı kodundan çağrılmalıdır
- FastComments verilerinize tam erişim sağlar
- tenant başına istek sınırlaması uygulanır

**Örnek kullanım**: Yönetimsel işlemler, toplu veri dışa aktarma, moderasyon araçları

**ÖNEMLİ**: API key'inizi istemci tarafı kodunda asla açığa çıkarmayın. API key'ler yalnızca sunucu tarafında kullanılmalıdır.