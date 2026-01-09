FastComments maps SAML user roles to internal permissions, enabling role-based access control for your organization.

### FastComments Rol Sistemi

FastComments, kullanıcıların erişim düzeylerini ve yeteneklerini belirleyen bir veya daha fazla role sahip olabileceği rol tabanlı bir izin sistemi kullanır.

### Mevcut FastComments Roller

#### Yönetimsel Roller

**fc-account-owner**
- **İzinler**: Tam yönetici erişimi
- **Yetenekler**: Tüm özellikler, faturalama yönetimi, kullanıcı yönetimi
- **Kullanım Örneği**: Birincil hesap yöneticileri ve sahipleri

**fc-admin-admin**  
- **İzinler**: Çoğu özelliğe yönetim erişimi
- **Yetenekler**: Kullanıcı yönetimi, yapılandırma, moderasyon. **Diğer yöneticileri yönetebilir.**
- **Kullanım Örneği**: İkincil yöneticiler ve BT personeli

**fc-billing-admin**
- **İzinler**: Faturalama ve abonelik yönetimi
- **Yetenekler**: Ödeme yöntemleri, faturalar, abonelik değişiklikleri
- **Kullanım Örneği**: Finans ekibi üyeleri ve fatura sorumluları

#### Uzmanlaşmış Roller

**fc-analytics-admin**
- **İzinler**: Analitik ve raporlama erişimi
- **Yetenekler**: Site istatistiklerini, kullanıcı etkileşim verilerini görüntüleme
- **Kullanım Örneği**: Pazarlama ekipleri ve veri analistleri

**fc-api-admin**
- **İzinler**: API erişimi ve yönetimi
- **Yetenekler**: API kimlik bilgileri, webhook yapılandırması
- **Kullanım Örneği**: Geliştiriciler ve teknik entegratörler

**fc-moderator**
- **İzinler**: Yorum moderasyon yetenekleri
- **Yetenekler**: Yorumları onaylama/red etme, spam yönetimi
- **Kullanım Örneği**: Topluluk moderatörleri ve içerik yöneticileri

### Rol Eşleme Yapılandırması

#### SAML Öznitelik Kaynakları

FastComments, farklı kimlik sağlayıcılarıyla uyumluluk sağlamak için çeşitli SAML öznitelik adlarından rol bilgisi kabul eder:

**Standart Öznitelik Adları**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS Öznitelikleri**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Rol Formatı Desteği

**Array Formatı** *(Tercih Edilen)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Virgülle Ayrılmış Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Tek Rol Formatı**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Kimlik Sağlayıcı Rol Yapılandırması

#### Microsoft Azure AD

1. **Uygulama Rolleri Yapılandırması**:
   - Azure AD uygulamanızda FastComments rollerini tanımlayın
   - Kullanıcıları uygun uygulama rollerine atayın
   - Atanmış rolleri içerecek şekilde claim yapılandırın

2. **Öznitelik Eşleme**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Grup Ataması**:
   - FastComments rol adlarıyla eşleşen gruplar oluşturun
   - Kullanıcıları uygun gruplara atayın
   - Öznitelik ifadelerini yapılandırın

2. **Öznitelik İfadesi**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Grup Eşleme**:
   - Organizasyon birimleri veya gruplar oluşturun
   - Grupları FastComments rol önekleriyle adlandırın
   - Öznitelik eşlemeyi yapılandırın

2. **Özel Öznitelikler**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Varsayılan Kullanıcı Davranışı

#### Rolleri Olmayan Kullanıcılar

Bir SAML kullanıcısının rolü yoksa veya tanınmayan roller içeriyorsa:
- Kullanıcı standart bir yorumcu olarak oluşturulur
- Yönetici erişimi verilmez
- Kendi yorumlarını gönderebilir ve yönetebilir
- Yönetici panosu özelliklerine erişemez

#### Rol Devralma

- Kullanıcılar aynı anda birden fazla role sahip olabilir
- İzinler kümülatiftir (en yüksek izin seviyesi uygulanır)
- IdP'deki rol değişiklikleri sonraki girişte yansıtılır

### SAML Kullanıcılarını Yönetme

#### Kullanıcı Oluşturma

Bir kullanıcı ilk kez SAML ile giriş yaptığında:
1. **Kullanıcı Hesabı**: E-posta kimlik olarak otomatik oluşturulur
2. **Rol Atama**: Roller SAML özniteliklerine göre uygulanır
3. **Profil Bilgileri**: Sağlanmışsa ad/soyad doldurulur
4. **İzin Aktivasyonu**: Roller hemen etkinleşir

#### Rol Güncellemeleri

Mevcut SAML kullanıcıları rol güncellemeleri alır:
1. **Giriş Tetiklemesi**: Rol güncellemeleri her SAML girişi sırasında gerçekleşir
2. **Anında Etki**: Yeni izinler hemen uygulanır
3. **Rol Kaldırma**: Kaldırılan roller otomatik olarak geri alınır
4. **Denetim Kaydı**: Rol değişiklikleri denetim günlüklerinde kaydedilir

### Özel Rol Eşleme

#### Kurumsal Özelleştirme

Belirli gereksinimleri olan kurumsal müşteriler için:
- Özel rol adları FastComments izinlerine eşlenebilir
- Karmaşık rol hiyerarşileri uygulanabilir
- Bölüme özel erişim kontrolleri yapılandırılabilir

Özel rol eşleme yapılandırmaları için FastComments desteği ile iletişime geçin.

#### Rol Doğrulama

FastComments gelen rolleri doğrular:
- Tanınmayan roller görmezden gelinir (reddedilmez)
- Bozuk rol öznitelikleri sorun giderme için kaydedilir
- SAML bildirimi rol bilgisi içermiyorsa kullanıcılar mevcut rollerini korur

### En İyi Uygulamalar

#### Rol Yönetimi

1. **Asgari Ayrıcalık İlkesi**: Gerekli en az izinleri atayın
2. **Düzenli Denetim**: Kullanıcı rolleri ve erişimi periyodik olarak gözden geçirin  
3. **Açık İsimlendirme**: IdP'nizde açıklayıcı grup adları kullanın
4. **Dokümantasyon**: Rol atamalarının dokümantasyonunu tutun

#### Güvenlik Hususları

1. **Rol Öznitelikleri**: Rol özniteliklerinin SAML yanıtlarında düzgün şekilde güvence altına alındığından emin olun
2. **Öznitelik Doğrulaması**: Yalnızca yetkili sistemlerin roller atayabildiğini doğrulayın
3. **Erişim İncelemeleri**: Yönetici rol atamalarını düzenli olarak gözden geçirin
4. **İzleme**: Rol değişikliklerini ve yönetici eylemlerini izleyin

### Rol Sorunlarını Giderme

#### Yaygın Sorunlar

**Roller Uygulanmadı**:
- SAML öznitelik adlarının desteklenen formatlarla eşleştiğini kontrol edin
- IdP'nin rol bilgisi gönderdiğini doğrulayın
- Rol değerlerinin FastComments rol adlarıyla tam olarak eşleştiğini onaylayın

**Erişim Reddedildi**:
- Kullanıcının IdP'de uygun role atanmış olduğunu doğrulayın
- Rol yazımını ve büyük/küçük harf duyarlılığını kontrol edin
- Rolün SAML yanıtında doğru formatlandığını onaylayın

**Eksik İzinler**:
- Rol tanımlarını ve gerekli izinleri gözden geçirin
- Çakışan rol atamalarını kontrol edin
- Kullanıcının rol değişikliklerinden sonra giriş yaptığını doğrulayın