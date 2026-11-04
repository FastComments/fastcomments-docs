FastComments SDK iki tür API uç noktası sağlar:

### PublicAPI - İstemci Tarafında Güvenli Uç Noktalar

`PublicAPI`, istemci tarafı kodundan (iOS/macOS uygulamaları) çağrılması güvenli olan uç noktaları içerir. Bu uç noktalar:
- API anahtarı gerektirmez
- Kimlik doğrulama için SSO token'ları kullanabilir
- Kullanıcı/cihaz başına hız sınırına tabidir
- Son kullanıcıya yönelik uygulamalar için uygundur

**Örnek kullanım senaryosu**: iOS uygulamanızda yorumları getirme ve oluşturma

### DefaultAPI - Sunucu Tarafı Uç Noktaları

`DefaultAPI`, API anahtarı gerektiren kimlik doğrulamalı uç noktaları içerir. Bu uç noktalar:
- FastComments API anahtarınızı gerektirir
- YALNIZCA sunucu tarafı kodundan çağrılmalıdır
- FastComments verilerinize tam erişim sağlar
- Kiracı (tenant) başına hız sınırına tabidir

**Örnek kullanım senaryosu**: Yönetimsel işlemler, toplu veri dışa aktarma, moderasyon araçları

**ÖNEMLİ**: API anahtarınızı asla istemci tarafı kodunda göstermeyin. API anahtarları yalnızca sunucu tarafında kullanılmalıdır.