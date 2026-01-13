FastComments'ta SAML yapılandırmasını tamamladıktan sonra, FastComments'ı kimlik sağlayıcınızda bir Hizmet Sağlayıcı (Service Provider) olarak ayarlamanız gerekir.

### Genel IdP Yapılandırması

Çoğu kimlik sağlayıcı, FastComments'ı bir SAML uygulaması olarak eklemek için aşağıdaki bilgilere ihtiyaç duyar:

#### Gerekli Hizmet Sağlayıcı Bilgileri

Bu değerler otomatik olarak oluşturulur ve FastComments SAML yapılandırma sayfanızda gösterilir:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Bu, FastComments örneğinizi benzersiz olarak tanımlar

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- IdP'nizin kimlik doğrulamadan sonra SAML yanıtlarını gönderdiği yer

**SP Metadata URL** *(IdP'niz destekliyorsa)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- XML formatında eksiksiz SAML yapılandırmasını sağlar

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- SAML kimlik doğrulamayı başlatmak için doğrudan bağlantı

### Gerekli SAML Öznitelikleri

Kimlik sağlayıcınızı bu öznitelikleri SAML yanıtlarıyla gönderecek şekilde yapılandırın:

#### Zorunlu Öznitelikler

**E-posta Adresi** *(Zorunlu)*
- **Öznitelik Adı**: `email`, `emailAddress`, veya `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Amaç**: Kullanıcının benzersiz tanımlanması ve bildirimler
- **Biçim**: Geçerli e-posta adresi

#### İsteğe Bağlı Öznitelikler

**Ad**
- **Öznitelik İsimleri**: `firstName`, `givenName`, veya `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Amaç**: Kullanıcı görüntü adı

**Soyad**
- **Öznitelik İsimleri**: `lastName`, `surname`, veya `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Amaç**: Kullanıcı görüntü adı

**Roller** *(Erişim kontrolü için önemli)*
- **Öznitelik İsimleri**: `roles`, `groups`, `memberOf`, veya özel öznitelik isimleri
- **Amaç**: FastComments rol ataması ve izinler
- **Biçim**: Rol dizelerinden oluşan dizi veya virgülle ayrılmış değerler

### Yaygın Kimlik Sağlayıcı Yapılandırmaları

#### Microsoft Azure AD

1. **Kurumsal Uygulama Ekle**
   - "FastComments" için arama yapın veya özel bir SAML uygulaması oluşturun
   - FastComments tarafından sağlanan SP bilgilerini kullanın

2. **Öznitelikleri Yapılandırın**
   - E-posta: `user.mail` veya `user.userprincipalname`
   - Ad: `user.givenname`
   - Soyad: `user.surname`
   - Roller: `user.assignedroles` veya dizin grupları

#### Okta

1. **SAML Uygulaması Oluştur**
   - "Create New App" seçeneğini kullanın ve SAML 2.0 seçin
   - FastComments SP bilgileri ile yapılandırın

2. **Öznitelik Bildirimleri**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` veya özel öznitelikler

#### Google Workspace

1. **SAML Uygulaması Ekle**
   - Apps > Web and mobile apps > Add App > Add custom SAML app yolunu izleyin
   - FastComments SP bilgileri ile yapılandırın

2. **Öznitelik Eşlemesi**
   - Email: Birincil e-posta
   - First Name: İlk isim
   - Last Name: Soy isim
   - Roles: Gruplar veya özel öznitelikler

#### Active Directory Federation Services (ADFS)

1. **Relying Party Trust Ekle**
   - FastComments metadata URL'sini veya manuel yapılandırmayı kullanın
   - Sağlanan SP bilgilerini yapılandırın

2. **Talep Kuralları**
   - E-posta: E-posta Adresi talebi
   - İsim: Name ID talebi
   - Roller: Grup üyeliği veya özel talepler

### Öznitelik Adı Esnekliği

FastComments, farklı IdP yapılandırmalarını karşılamak için rol bilgilerini birden fazla öznitelik isminden kabul eder:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Bu esneklik, belirli öznitelik adlandırma kurallarını gerektirmeden çeşitli kimlik sağlayıcılarla uyumluluğu garanti eder.

### Yapılandırmanızı Test Etme

Kimlik sağlayıcınızı yapılandırdıktan sonra:

1. IdP yapılandırmasını kaydedin
2. Adanmış bir test kullanıcı hesabı ile test edin
3. Özniteliklerin doğru gönderildiğini doğrulayın
4. Rollerinin doğru şekilde eşlendiğini kontrol edin
5. Kimlik doğrulama akışının başarıyla tamamlandığından emin olun

Çoğu kimlik sağlayıcı, yapılandırmayı üretim kullanıcılarına dağıtmadan önce doğrulamak için SAML test araçları sunar.