Bu kılavuz yaygın SAML kimlik doğrulama sorunlarını ve çözümlerini kapsar.

### Sertifika ve Güvenlik Sorunları

#### Geçersiz Sertifika Hatası

**Belirtiler**:
- "Certificate validation failed" hatası
- Kullanıcılar SAML kimlik doğrulamayı tamamlayamıyor
- SAML yanıtları reddediliyor

**Yaygın Nedenler**:
- Sertifika formatı yanlış
- Sertifika süresi dolmuş
- Yanlış sertifika sağlanmış
- Sertifikada ekstra karakterler veya boşluk var

**Çözümler**:
1. **Sertifika Formatını Doğrulayın**:
   - Sertifikanın `-----BEGIN CERTIFICATE-----` ve `-----END CERTIFICATE-----` işaretlerini içerdiğinden emin olun
   - Herhangi bir ekstra boşluk veya satır sonunu kaldırın
   - Sertifikayı IdP meta verilerinden veya yapılandırmasından doğrudan kopyalayın

2. **Sertifika Geçerliliğini Kontrol Edin**:
   - Sertifikanın süresinin dolmadığını doğrulayın
   - Sertifikanın doğru IdP için olduğunu onaylayın
   - Formatı kontrol etmek için çevrimiçi sertifika doğrulaycıları kullanın

3. **Sertifikayı Yeniden İndirin**:
   - IdP'den taze sertifika indirin
   - Mümkünse IdP meta veri URL'sini kullanın
   - Sertifikanın mevcut IdP yapılandırmasıyla eşleştiğini doğrulayın

#### İmza Doğrulama Başarısız

**Belirtiler**:
- SAML bildirisinin (assertion) imza doğrulama hataları
- IdP girişinden sonra kimlik doğrulama başarısız oluyor
- "Invalid signature" hata mesajları

**Çözümler**:
1. **Algoritma Uyumsuzluğu**:
   - FastComments'daki imza algoritmasının IdP ile eşleştiğini kontrol edin
   - Farklı imza algoritmalarını deneyin (SHA-256, SHA-1, SHA-512)
   - Özümleme (digest) algoritmasının IdP yapılandırmasıyla eşleştiğini doğrulayın

2. **Sertifika Sorunları**:
   - Doğru imzalama sertifikasının yapılandırıldığından emin olun
   - Sertifikanın IdP tarafından kullanılan özel anahtarla (private key) ilişkili olduğunu doğrulayın
   - IdP'de sertifika rotasyonu olup olmadığını kontrol edin

### Yapılandırma Sorunları

#### Yanlış Entity ID veya ACS URL

**Belirtiler**:
- IdP "Unknown Service Provider" bildiriyor
- SAML yanıtları yanlış uç noktaya gidiyor
- Kimlik doğrulama tamamlanmıyor

**Çözümler**:
1. **SP Bilgilerini Doğrulayın**:
   - FastComments yapılandırmasından tam Entity ID'yi kopyalayın
   - ACS URL'sinin şu formatla eşleştiğini doğrulayın: `https://fastcomments.com/saml/callback/{tenant-id}`
   - tenant ID içinde yazım hatası olup olmadığını kontrol edin

2. **IdP Yapılandırması**:
   - IdP'yi doğru SP Entity ID ile güncelleyin
   - Doğru ACS/Reply URL'yi yapılandırın
   - IdP binding ayarlarını doğrulayın (HTTP-POST tercih edilir)

#### Eksik veya Yanlış Öznitelikler

**Belirtiler**:
- Kullanıcılar uygun rollere sahip olmadan oluşturuluyor
- Kullanıcı profil bilgileri eksik
- "Email required" hataları

**Çözümler**:
1. **E-posta Özniteliği**:
   - IdP'nin e-posta özniteliğini gönderdiğinden emin olun
   - Öznitelik adı eşlemesini kontrol edin (email, emailAddress, vb.)
   - E-posta değerinin geçerli bir e-posta adresi olduğunu doğrulayın

2. **Rol Öznitelikleri**:
   - IdP'nin rol/grup bilgisini gönderdiğini doğrulayın
   - Rol özniteliği adlarının FastComments beklentileriyle eşleştiğini kontrol edin
   - Rol değerlerinin FastComments rol adlarıyla tam olarak eşleştiğini onaylayın

3. **Öznitelik Formatı**:
   - Rol formatlarını dizi (array) ve virgülle ayrılmış olarak test edin
   - Öznitelik değerlerinde ekstra boşluk olmadığından emin olun
   - Rol adlarında büyük/küçük harf duyarlılığını kontrol edin

### Kimlik Doğrulama Akışı Sorunları

#### Yönlendirme Döngüsü

**Belirtiler**:
- Tarayıcı FastComments ve IdP arasında sonsuz yönlendirme yapıyor
- Kimlik doğrulama hiçbir zaman tamamlanmıyor
- Tarayıcı geliştirici araçlarında birden fazla yönlendirme gösteriliyor

**Çözümler**:
1. **SP Yapılandırmasını Kontrol Edin**:
   - Entity ID'nin IdP yapılandırmasıyla tam olarak eşleştiğini doğrulayın
   - ACS URL'sinin IdP'de doğru yapılandırıldığından emin olun
   - URL'lerde sonundaki eğik çizgilere (trailing slashes) dikkat edin

2. **Oturum (Session) Sorunları**:
   - Tarayıcı çerezlerini temizleyip tekrar deneyin
   - Gizli/özel tarayıcı penceresinde test edin
   - Oturum zaman aşımı ayarlarını kontrol edin

#### Kimlik Doğrulamadan Sonra Erişim Reddedildi

**Belirtiler**:
- SAML kimlik doğrulaması başarılı oluyor
- Kullanıcı FastComments'e yönlendiriliyor
- "Access denied" veya izin hatası görüntüleniyor

**Çözümler**:
1. **Rol Ataması**:
   - Kullanıcının IdP'de uygun rollere sahip olduğunu doğrulayın
   - Rol özniteliğinin SAML yanıtında gönderildiğini kontrol edin
   - Rol adlarının FastComments gereksinimleriyle tam olarak eşleştiğini onaylayın

2. **Paket Sınırlamaları**:
   - Hesabın Flex veya Pro planında olduğunu doğrulayın
   - SAML özelliğinin pakette etkin olup olmadığını kontrol edin
   - Paket SAML içeriyorsa ancak özellik kullanılamıyorsa destek ile iletişime geçin

### Kimlik Sağlayıcıya Özgü Sorunlar

#### Microsoft Azure AD

**Yaygın Sorunlar**:
- Uygulama rol atamaları token'larda yansımıyor
- Talepler (claims) düzgün gönderilmiyor
- Kullanıcı atama gereksinimleri

**Çözümler**:
- FastComments uygulamasına kullanıcı atamasını kontrol edin
- Uygulama rollerinin doğru yapılandırıldığını doğrulayın
- Gerekli özniteliklerin dahil edildiğinden emin olmak için claims eşlemesini kontrol edin

#### Okta

**Yaygın Sorunlar**:
- Grup filtreleri düzgün çalışmıyor
- Öznitelik ifadeleri yanlış yapılandırılmış
- Uygulama atama problemleri

**Çözümler**:
- Öznitelik ifadesi (attribute statement) yapılandırmasını gözden geçirin
- Grup atama ve filtreleme kurallarını kontrol edin
- Uygulamanın uygun kullanıcı/gruplara atandığından emin olun

#### Google Workspace

**Yaygın Sorunlar**:
- Özel öznitelikler doğru eşlenmiyor
- Grup üyeliği gönderilmiyor
- SAML uygulama yapılandırma hataları

**Çözümler**:
- Rol öznitelikleri için özel şema yapılandırın
- Grup üyeliği aktarımını kontrol edin
- SAML uygulaması öznitelik eşlemesini doğrulayın

### Ağ ve Bağlantı Sorunları

#### Zaman Aşımı Hataları

**Belirtiler**:
- Kimlik doğrulama işlemi zaman aşımına uğruyor
- "Request timeout" veya benzeri hatalar
- Yavaş kimlik doğrulama akışı

**Çözümler**:
1. **Ağ Bağlantısı**:
   - Güvenlik duvarı kurallarının FastComments ile iletişime izin verdiğini kontrol edin
   - fastcomments.com için DNS çözümlemesini doğrulayın
   - IdP'den FastComments'e ağ bağlantısını test edin

2. **Performans Sorunları**:
   - IdP yanıt sürelerini kontrol edin
   - Sertifika zinciri doğrulamasının yavaş olup olmadığını doğrulayın
   - IdP ile kullanıcılar arasındaki ağ gecikmesini (latency) göz önünde bulundurun

#### SSL/TLS Sorunları

**Belirtiler**:
- Kimlik doğrulama sırasında sertifika uyarıları
- SSL el sıkışma (handshake) hataları
- "Secure connection failed" hataları

**Çözümler**:
- Tüm SAML uç noktalarının HTTPS kullandığından emin olun
- İlgili tüm alan adları için sertifika geçerliliğini kontrol edin
- TLS sürümü uyumluluğunu doğrulayın

### Hata Ayıklama ve Kayıtlar (Logging)

#### Hata Ayıklama Bilgilerini Etkinleştirme

1. **Tarayıcı Geliştirici Araçları**:
   - SAML akışı sırasında Ağ (Network) sekmesini izleyin
   - JavaScript hataları için Konsol'u (Console) kontrol edin
   - SAML POST isteklerini inceleyin (görünürse)

2. **IdP Kayıtları**:
   - IdP'de SAML hata ayıklamayı etkinleştirin
   - SAML istek/yanıt ayrıntıları için IdP günlüklerini inceleyin
   - Öznitelik eşleme sorunlarını kontrol edin

#### Yaygın Günlük Mesajları

**FastComments Günlükleri**:
- "SAML config not found" - SAML etkin değil veya yanlış yapılandırılmış
- "Invalid certificate" - Sertifika doğrulaması başarısız
- "Missing email attribute" - Gerekli e-posta SAML yanıtında sağlanmamış

**IdP Günlükleri**:
- "Unknown service provider" - Entity ID uyuşmazlığı
- "Invalid ACS URL" - Assertion Consumer Service URL yanlış
- "User not assigned" - Kullanıcının SAML uygulamasına erişimi yok

### Yardım Alma

#### Toplamanız Gereken Bilgiler

Destek ile iletişime geçerken sağlayın:
- Kesin hata mesajları ve zaman damgaları
- SAML yapılandırma ayrıntıları (gizli veriler olmadan)
- IdP türü ve sürümü
- Sorunu yeniden üretme adımları
- Tarayıcı ve ağ bilgileri

#### FastComments Desteği

SAML ile ilgili sorunlar için:
1. [support portalını](https://fastcomments.com/auth/my-account/help) kullanın
2. tenant ID ve etkilenen kullanıcı e-posta adreslerini ekleyin
3. Hata mesajlarını ve yapılandırma ayrıntılarını sağlayın
4. IdP türünü ve yapılandırma yaklaşımını belirtin

#### IdP Desteği

IdP'ye özgü sorunlar için:
- SAML hata ayıklama için IdP belgelerine başvurun
- Yapılandırma problemleri için IdP destek kanallarını kullanın
- Yaygın sorunlar için IdP topluluk forumlarından yararlanın

### Önleme İpuçları

#### En İyi Uygulamalar

1. **Kapsamlı Test Yapın**:
   - Yapılandırma değişikliklerini üretim dışı ortamda test edin
   - Birden fazla test kullanıcısıyla doğrulayın
   - Çalışan yapılandırmaları belgelendirin

2. **Düzenli İzleme**:
   - SAML kimlik doğrulama hataları için izleme kurun
   - Sertifika son kullanma tarihlerini gözden geçirin
   - IdP yapılandırma değişikliklerini izleyin

3. **Dokümantasyon**:
   - SAML yapılandırmasını belgeleyin
   - Herhangi bir özel yapılandırmayı veya geçici çözümü kaydedin
   - IdP yöneticilerinin iletişim bilgilerini güncel tutun

#### Proaktif Bakım

1. **Sertifika Yönetimi**:
   - Sertifika son kullanma tarihlerini izleyin
   - Sertifika döndürme (rotation) prosedürlerini planlayın
   - Sertifika güncellemelerini süresi dolmadan önce test edin

2. **Yapılandırma İncelemeleri**:
   - Düzenli olarak SAML yapılandırmasını gözden geçirin
   - IdP yapılandırmasının güncel kaldığını doğrulayın
   - Yapılan değişiklikler doğrultusunda dokümantasyonu güncelleyin