### 401 Yetkisiz Hatalar

1. **API anahtarınızı kontrol edin**: Doğru API anahtarını FastComments kontrol panelinizden kullandığınızdan emin olun
2. **Kiracı kimliğini doğrulayın**: Kiracı kimliğinin hesabınızla eşleştiğinden emin olun
3. **API anahtarı formatı**: API anahtarı, paylaşılan yapılandırmadaki `x-api-key` başlığı olarak ayarlanmalıdır:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Yanlış API Kullanımı**: Doğrulanmış çağrılar için `DefaultAPI`'yi (`PublicAPI` değil) kullandığınızdan emin olun

### SSO Token Sorunları

1. **Üretim için güvenli mod kullanın**: Üretimde her zaman API anahtarınızla `FastCommentsSSO.createSecure()` kullanın
2. **Yalnızca sunucu tarafında**: Güvenli SSO tokenlerini sunucunuzda oluşturun, API anahtarınızı istemcilere asla açığa çıkarmayın
3. **Kullanıcı verilerini kontrol edin**: Tüm gerekli alanların (id, email, username) sağlandığından emin olun
4. **Token süresi dolması**: Güvenli SSO tokenleri bir zaman damgası içerir ve süresi dolabilir. Gerektiğinde yeni tokenler oluşturun.

### SSL/TLS Hataları

1. Uygulamanızın Info.plist dosyasının fastcomments.com için HTTPS bağlantılarına izin verdiğinden emin olun
2. Bağlantıyı engelleyebilecek App Transport Security istisnalarını kullanmadığınızdan emin olun