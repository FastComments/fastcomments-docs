FastComments'ta SAML etkinleştirildiğinde, sistem kimlik sağlayıcınızda yapılandırmanız gereken Hizmet Sağlayıcı (SP) bilgilerini otomatik olarak oluşturur.

### Hizmet Sağlayıcı Bilgilerine Erişim

SP bilgileri, SAML kimlik doğrulamayı etkinleştirdikten sonra SAML yapılandırma sayfanızda görüntülenir. Bu bilgiler, kimlik sağlayıcınızın SAML güven ilişkisini oluşturmak için ihtiyaç duyduğu tüm ayrıntıları içerir.

### Hizmet Sağlayıcı Uç Noktaları

#### SP Varlık Kimliği / Audience
**Amaç**: FastComments örneğinizi hizmet sağlayıcı olarak benzersiz şekilde tanımlar  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Kullanım**: Bunu IdP'nizde Varlık Kimliği veya Audience olarak yapılandırın

Bu tanımlayıcı, SAML yanıtlarının belirli FastComments kiracınıza yönelik olduğunu garanti eder ve diğer örnekler tarafından kabul edilmesini engeller.

#### Assertion Consumer Service (ACS) URL
**Amaç**: Kullanıcı kimlik doğrulamasından sonra IdP'nizin SAML yanıtlarını gönderdiği uç nokta  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Kullanım**: Bunu IdP'nizde ACS URL'si veya Reply URL olarak yapılandırın

Başarılı kimlik doğrulama sonrası kullanıcıların, kullanıcı bilgilerini içeren SAML beyanı ile birlikte yönlendirildiği yerdir.

#### SP Metadata URL
**Amaç**: Standart XML formatında eksiksiz SAML yapılandırmasını sağlar  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Kullanım**: Bazı IdP'ler bu URL'yi kullanarak yapılandırmayı otomatik olarak içe aktarabilir

Metadata URL'si, XML formatında gerekli tüm SP bilgilerini içerir ve uyumlu kimlik sağlayıcılarının yapılandırmasını otomatikleştirmeyi kolaylaştırır.

#### SAML Login URL
**Amaç**: Kiracınız için SAML kimlik doğrulamayı başlatmak üzere doğrudan bağlantı  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Kullanım**: Kullanıcıları doğrudan SAML kimlik doğrulamaya yönlendirmek veya akışı test etmek için kullanın

Bu URL'yi SAML kimlik doğrulamayı test etmek veya kullanıcılara SAML ile oturum açmaları için doğrudan bir bağlantı sağlamak amacıyla kullanabilirsiniz.

### SAML Bağlama Desteği

FastComments aşağıdaki SAML bağlama yöntemlerini destekler:

#### HTTP-POST Bağlaması
- **Birincil Yöntem**: SAML yanıtları için en yaygın bağlama  
- **Güvenlik**: SAML yanıtı ACS URL'sine HTTP POST ile gönderilir  
- **Kullanım**: Üretim dağıtımları için önerilir

#### HTTP-Redirect Bağlaması
- **Alternatif Yöntem**: SAML yanıtı HTTP yönlendirmesi (redirect) ile gönderilir  
- **Sınırlamalar**: URL uzunluğu kısıtlamaları nedeniyle sınırlı yük boyutu  
- **Kullanım**: Desteklenir ancak HTTP-POST tercih edilir

### Name ID Politikası

FastComments, SAML isteklerinde aşağıdaki Name ID politikasını yapılandırır:

- **Varsayılan Format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternatif Formatlar**: Persistent, Transient, Unspecified (yapılandırılabilir)  
- **Gereklilik**: Birincil kullanıcı tanımlayıcısı olarak e-posta adresi kullanılır

### SAML İstek Öznitelikleri

SAML kimlik doğrulamasını başlatırken FastComments şu özelliklerle istekler gönderir:

#### İstek İmzalama
- **Durum**: Opsiyonel (yapılandırılabilir)  
- **Algoritma**: Yapılandırılmış imza algoritması ile eşleşir  
- **Sertifika**: İstek imzalama etkinse kiraca özel sertifika kullanılır

#### İstenen Öznitelikler
FastComments, SAML AuthnRequest'lerde aşağıdaki öznitelikleri talep eder:

- **Email**: Kullanıcı tanımlaması için gerekli  
- **First Name**: Görüntüleme amaçlı isteğe bağlı  
- **Last Name**: Görüntüleme amaçlı isteğe bağlı  
- **Roles/Groups**: Erişim kontrolü ve izinler için isteğe bağlı

### SP Bilgilerinin Kopyalanması

SAML yapılandırma sayfası, SP bilgilerini panonuza otomatik olarak kopyalayan tıklanabilir alanlar sağlar:

1. Herhangi bir SP bilgi alanına (Entity ID, ACS URL, vb.) tıklayın  
2. Değer otomatik olarak panonuza kopyalanır  
3. Değeri kimlik sağlayıcısı yapılandırmanıza yapıştırın  
4. Başarılı kopyalamayı gösteren kısa bir vurgu görüntülenir

Bu, SP bilgilerini IdP'nize doğru bir şekilde aktarmayı yazma hatası olmadan kolaylaştırır.

### SP Sertifika Bilgileri

#### Sertifika Kullanımı
- **Amaç**: İletişimi şifreler ve SP kimliğini doğrular  
- **Döndürme**: Sertifikalar FastComments tarafından otomatik olarak yönetilir  
- **Erişim**: Genel sertifikalar metadata URL'si üzerinden erişilebilir

#### Sertifika Ayrıntıları
- **Algoritma**: RSA-2048 veya daha yüksek  
- **Geçerlilik**: Sertifikalar süresi dolmadan önce otomatik olarak yenilenir  
- **Dağıtım**: Standart SAML metadata aracılığıyla sağlanır

### SP Yapılandırma Sorun Giderme

Kimlik sağlayıcınız SP bilgileriyle ilgili sorun bildiriyorsa:

1. **URL'leri Doğrulayın**: Tüm URL'lerin HTTPS kullandığından ve doğru kiracı kimliğini içerdiğinden emin olun  
2. **Metadata'yı Kontrol Edin**: Yapılandırmayı doğrulamak için metadata URL'sini kullanın  
3. **Bağlantıyı Test Edin**: IdP'nizin FastComments uç noktalarına erişebildiğinden emin olun  
4. **Formatı Doğrulayın**: IdP'nizin SP bilgi formatını desteklediğini onaylayın

Yaygın sorunlar şunları içerir:
- URL'lerde yanlış kiracı kimliği  
- IdP ile FastComments arasındaki ağ bağlantı problemleri  
- IdP'nin farklı URL formatları veya ek yapılandırma seçenekleri beklemesi