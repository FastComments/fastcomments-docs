### 401 Yetkisiz Hatalar

Kimlik doğrulamalı API'yi kullanırken 401 hataları alıyorsanız:

1. **API anahtarınızı kontrol edin**: FastComments kontrol panelinizden doğru API anahtarını kullandığınızdan emin olun
2. **Tenant ID'sini doğrulayın**: Tenant ID'nin hesabınızla eşleştiğinden emin olun
3. **API anahtarı formatı**: API anahtarı API istemcisinde ayarlanmış olmalıdır:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Yanlış API'yi mı kullanıyorsunuz**: Kimlik doğrulamalı çağrılar için `DefaultAPI` (not `PublicAPI`) kullandığınızdan emin olun

### SSO Jeton Sorunları

SSO jetonları çalışmıyorsa:

1. **Üretimde güvenli modu kullanın**: Üretimde her zaman API anahtarınızla birlikte `FastCommentsSSO.createSecure()` kullanın
2. **Sadece sunucu tarafında**: Güvenli SSO jetonlarını sunucunuzda oluşturun, API anahtarınızı istemcilere asla açığa çıkarmayın
3. **Kullanıcı verilerini kontrol edin**: Gerekli tüm alanların (id, email, username) sağlandığından emin olun
4. **Jeton süresi**: Güvenli SSO jetonları bir zaman damgası içerir ve süresi dolabilir. Gerektiğinde yeni jetonlar oluşturun.

### SSL/TLS Hataları

SSL/TLS hatalarıyla karşılaşırsanız:

1. Uygulamanızın Info.plist dosyasının fastcomments.com'a HTTPS bağlantılarına izin verdiğinden emin olun
2. Bağlantıyı engelleyebilecek App Transport Security istisnalarını kullanmadığınızdan emin olun