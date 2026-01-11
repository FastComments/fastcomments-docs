### 401 Yetkisiz Hatalar

Yetkilendirilmiş API'yi kullanırken 401 hatası alıyorsanız:

1. **API anahtarınızı kontrol edin**: FastComments panelinizden doğru API anahtarını kullandığınızdan emin olun
2. **Tenant ID'sini doğrulayın**: Tenant ID'sinin hesabınızla eşleştiğinden emin olun
3. **API anahtarı formatı**: API anahtarı Configuration içinde geçirilmelidir:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO Jeton Sorunları

SSO jetonları çalışmıyorsa:

1. **Üretimde güvenli modu kullanın**: Üretimde API anahtarınızla birlikte her zaman `FastCommentsSSO::new_secure()` kullanın
2. **Sadece sunucu tarafı**: SSO jetonlarını sunucunuzda oluşturun, API anahtarınızı asla istemcilere açığa çıkarmayın
3. **Kullanıcı verilerini kontrol edin**: Gerekli tüm alanların (id, email, username) sağlandığından emin olun

### Asenkron Çalışma Zamanı Hataları

SDK asenkron işlemler için tokio kullanır. Şunları yaptığınızdan emin olun:

1. Bağımlılıklarınıza tokio'yu ekleyin:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. tokio çalışma zamanını kullanın:
```rust
#[tokio::main]
async fn main() {
    // Asenkron kodunuz burada
}
```