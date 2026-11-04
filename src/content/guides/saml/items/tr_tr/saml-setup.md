FastComments'te SAML kimlik doğrulamasını kurmak hem yönetici panelinizde yapılandırma hem de kimlik sağlayıcınızda kurulum gerektirir.

### Ön Koşullar

SAML'yi yapılandırmadan önce şunlara sahip olduğunuzdan emin olun:

- Bir FastComments Flex veya Pro planı (SAML, Creators planında mevcut değildir)
- FastComments hesabınıza yönetici erişimi
- Kimlik sağlayıcınıza yönetici erişimi
- IdP'nizin SAML meta verileri veya sertifika bilgileri

### SAML Yapılandırmasına Erişim

1. [FastComments yönetici panelinize](https://fastcomments.com/auth/my-account) giriş yapın
2. Sol kenar çubuğunda **API/SSO Settings** bölümüne gidin
3. **SAML Config** düğmesine tıklayın

SAML Config düğmesini görmüyorsanız, şunları doğrulayın:
- Hesabınızın gerekli pakete (Flex veya Pro) sahip olduğu
- Yönetici izinlerine sahip olduğunuz
- Kullanıcınızın API Admin veya Admin Admin rollerine sahip olduğu

### Temel SAML Yapılandırması

#### SAML Kimlik Doğrulamasını Etkinleştirme

1. **Enable SAML Authentication** onay kutusunu işaretleyin
2. Bu, kiracınız (tenant) için SAML'yi etkinleştirir ve yapılandırma alanlarını kullanılabilir hale getirir

#### Zorunlu Alanlar

**IdP Single Sign-On URL** *(Zorunlu)*
- Kullanıcıların kimlik doğrulaması için yönlendirileceği URL
- Genellikle kimlik sağlayıcınız tarafından sağlanır
- Örnek: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Zorunlu)*
- Kimlik sağlayıcınızdan gelen genel sertifika
- SAML yanıtlarının gerçekliğini doğrulamak için kullanılır
- BEGIN/END işaretleriyle birlikte tam sertifikayı içermelidir
- Örnek biçim:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### İsteğe Bağlı Alanlar

**IdP Entity ID / Issuer**
- Kimlik sağlayıcınızı tanımlar
- Boş bırakılırsa, varsayılan olarak FastComments URL'niz kullanılır
- IdP'nizde yapılandırılmış issuer ile eşleşmelidir

### Gelişmiş Yapılandırma

#### Güvenlik Ayarları

**Signature Algorithm**
- Varsayılan olarak SHA-256 (önerilir)
- Seçenekler: SHA-1, SHA-256, SHA-512
- IdP'nizin yapılandırmasıyla eşleşmelidir

**Digest Algorithm**
- Varsayılan olarak SHA-256 (önerilir)
- SAML yanıtlarında özet (digest) hesaplaması için kullanılır
- IdP'nizin yapılandırmasıyla eşleşmelidir

**Name ID Format**
- Varsayılan olarak E-posta Adresi biçimindedir
- Kullanıcı tanımlayıcılarının nasıl biçimlendirileceğini belirler
- Yaygın seçenekler: E-posta Adresi, Persistent, Transient

#### Şifreleme (İsteğe Bağlı)

**Private Key for Decryption**
- Yalnızca IdP'niz SAML assertion'larını şifreliyorsa gereklidir
- Şifre çözme için kullanılan özel anahtarınızı yapıştırın
- Çoğu dağıtım assertion şifrelemesi gerektirmez

### Yapılandırmayı Kaydetme

1. Tüm ayarları doğruluk açısından gözden geçirin
2. **Save SAML Configuration** düğmesine tıklayın
3. Sistem yapılandırmanızı doğrulayacaktır
4. Başarılı olursa, bir onay mesajı göreceksiniz

### Sonraki Adımlar

FastComments SAML yapılandırmanızı kaydettikten sonra:

1. Service Provider bilgilerini kullanarak kimlik sağlayıcınızı yapılandırın
2. Kimlik doğrulama akışını test edin
3. Gerektiği şekilde kullanıcı rollerini ve izinlerini ayarlayın

IdP yapılandırmanız için gereken Service Provider bilgileri, SAML etkinleştirildikten sonra görüntülenecektir.