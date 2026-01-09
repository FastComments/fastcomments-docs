Setting up SAML authentication in FastComments requires both configuration in your admin dashboard and setup in your identity provider.

### Önkoşullar

SAML yapılandırmasına başlamadan önce şunlara sahip olduğunuzdan emin olun:

- Bir FastComments Flex veya Pro planı (SAML Creators planında mevcut değildir)
- FastComments hesabınızda yönetici erişimi
- Kimlik sağlayıcınızda yönetici erişimi
- IdP'nizin SAML meta verileri veya sertifika bilgileri

### SAML Yapılandırmasına Erişim

1. [FastComments yönetici panelinize](https://fastcomments.com/auth/my-account) giriş yapın
2. Sol kenar çubuğunda **API/SSO Ayarları** bölümüne gidin
3. **SAML Yapılandırması** düğmesine tıklayın

SAML Yapılandırması düğmesini görmüyorsanız, şunları doğrulayın:
- Hesabınızın gereken pakete (Flex veya Pro) sahip olduğundan
- Yönetici izinlerine sahip olduğunuzdan
- Kullanıcınızın API Admin veya Admin Admin rollerine sahip olduğundan

### Temel SAML Yapılandırması

#### SAML Kimlik Doğrulamayı Etkinleştirme

1. **SAML Kimlik Doğrulamayı Etkinleştir** onay kutusunu işaretleyin
2. Bu, kiracınız için SAML'i etkinleştirir ve yapılandırma alanlarını kullanılabilir hale getirir

#### Gerekli Alanlar

**IdP Tek Oturum Açma URL'si** *(Gerekli)*
- Kullanıcıların kimlik doğrulama için yönlendirileceği URL
- Genellikle kimlik sağlayıcınız tarafından verilir
- Örnek: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Sertifikası** *(Gerekli)*
- Kimlik sağlayıcınızdan alınan herkese açık sertifika
- SAML yanıtlarının doğruluğunu teyit etmek için kullanılır
- BEGIN/END işaretçilerini içeren tam sertifikayı içermelidir
- Örnek format:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### İsteğe Bağlı Alanlar

**IdP Varlık Kimliği / Issuer**
- Kimlik sağlayıcınızı tanımlar
- Boş bırakılırsa, varsayılan olarak FastComments URL'nize ayarlanır
- IdP'nizde yapılandırılan issuer ile eşleşmelidir

### Gelişmiş Yapılandırma

#### Güvenlik Ayarları

**İmza Algoritması**
- Varsayılan olarak SHA-256 (önerilen)
- Seçenekler: SHA-1, SHA-256, SHA-512
- IdP'nizin yapılandırmasıyla eşleşmelidir

**Özet (Digest) Algoritması**
- Varsayılan olarak SHA-256 (önerilen)
- SAML yanıtlarında özet hesaplama için kullanılır
- IdP'nizin yapılandırmasıyla eşleşmelidir

**Name ID Formatı**
- Varsayılan olarak E-posta Adresi formatı
- Kullanıcı tanımlayıcılarının nasıl biçimlendirileceğini belirler
- Yaygın seçenekler: E-posta Adresi, Kalıcı (Persistent), Geçici (Transient)

#### Şifreleme (İsteğe Bağlı)

**Şifre Çözme için Özel Anahtar**
- Sadece IdP'niz SAML doğrulamalarını şifreliyorsa gereklidir
- Şifre çözme için kullanılan özel anahtarınızı yapıştırın
- Çoğu dağıtımda assertion şifrelemesine gerek yoktur

### Yapılandırmayı Kaydetme

1. Tüm ayarları doğruluk açısından gözden geçirin
2. **SAML Yapılandırmasını Kaydet** düğmesine tıklayın
3. Sistem yapılandırmanızı doğrulayacaktır
4. Başarılı olursa, bir onay mesajı göreceksiniz

### Sonraki Adımlar

FastComments SAML yapılandırmanızı kaydettikten sonra:

1. Service Provider bilgilerini kullanarak kimlik sağlayıcınızı yapılandırın
2. Kimlik doğrulama akışını test edin
3. Gerekirse kullanıcı rolleri ve izinlerini ayarlayın

IdP yapılandırmanız için gerekli Service Provider bilgileri SAML etkinleştirildiğinde gösterilecektir.