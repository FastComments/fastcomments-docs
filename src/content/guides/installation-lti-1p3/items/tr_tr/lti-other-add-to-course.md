FastComments platforma kaydedildikten sonra eğitmenler, platformun standart dış araç akışlarını kullanarak onu ders içeriğine ekler. Bu sayfa Sakai 23.x ve Schoology Enterprise'ı kapsar.

#### Genel Erişimi Kısıtlayın (Önerilen)

Varsayılan olarak, FastComments yorum verileri her iki platformda da herkese açık okunabilir durumdadır. Bir kişi bir konunun URL’sini veya API uç noktasını tahmin edebilirse, Sakai veya Schoology dışında olsalar bile yorumları görebilirler. Ders tartışmaları için görüntülemeyi büyük olasılıkla yalnızca derse kayıtlı öğrencilerle sınırlamak istersiniz.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Widget özelleştirme sayfanızı</a> açın ve **Yorumları Görmek İçin SSO Gerekli** etkin olan bir kural oluşturun, ardından güvenlik düzeyini **Güvenli SSO** olarak ayarlayın, böylece konular yalnızca imzalı LTI başlatmasıyla yüklenebilir.

Tam adım adım kılavuz için, kuralı tek bir alan adına veya sayfaya nasıl sınırlayacağınızı da içeren [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) sayfasına bakın.

#### Sakai

**1. FastComments'i bir siteye ekleyin**

Site yöneticisi aracı site bazında etkinleştirir:

1. Siteyi açın ve sol gezinti menüsünde **Site Info**'ya tıklayın.
2. **Manage Tools**'a tıklayın.
3. **External Tools** listesini kaydırın ve **FastComments**'i açık duruma getirin.
4. **Continue**'a tıklayın, araç listesini gözden geçirin, ardından **Finish**'e tıklayın.

FastComments artık sitede sol gezinti öğesi olarak görünür.

**2. Sol gezinme girişinin sırasını değiştirme**

**Site Info** > **Tool Order** bölümüne gidin. **FastComments**'i istediğiniz konuma sürükleyin ve **Save**'e tıklayın. Bu ekranda gezinme etiketinin adını da değiştirebilir ve öğrencilerden gizleyebilirsiniz.

**3. Lessons sayfasına satır içi gömme**

FastComments'i bağımsız bir sol gezinme aracı olarak değil, Lessons sayfasının içine doğrudan yerleştirmek için:

1. Sitede **Lessons** aracını açın.
2. **Add Content** > **Add External Tool**'a tıklayın.
3. Listeden **FastComments**'i seçin.
4. FastComments kayıt sırasında Deep Linking'i duyurduysa, Sakai aracın içerik seçicisini açar ve konuyu seçmenize veya etiketlemenize olanak tanır. Deep Linking duyurulmadıysa, Sakai varsayılan bir başlatma bağlantısı ekler.
5. Lessons öğesini kaydedin.

Her gömülü örnek, o kaynak bağlantısına yönelik kendi konusu ile ayrı bir konu alır.

**4. Öğrenci erişimi için izin ayarları**

Sakai, dış araç başlatmalarını Realms üzerinden sınırlar. Öğrencilerin FastComments'i başlatabildiğini doğrulamak için:

1. Sakai yöneticisi olarak oturum açın ve **Administration Workspace** > **Realms**'i açın.
2. İlgili realm'i açın (örneğin `!site.template.course` veya belirli site realm'i).
3. `access` rolünün `lti.launch` etkinleştirildiğini ve **external.tools** grubundaki rol izinlerinin verildiğini onaylayın.
4. Realm'i kaydedin.

Site düzeyinde geçersiz kılmalar için, site yöneticisi **Site Info** > **Tool Order** üzerinden rol başına araç görünürlüğünü FastComments'i gizleyerek veya göstererek ayarlayabilir.

**5. Öğrencilerin gördükleri**

Öğrenciler FastComments sol gezinme öğesine tıklar (veya gömülü Lessons bloğuna kaydırırlar) ve doğrudan dizili yorum görünümüne gelirler. SSO otomatiktir: Sakai, kullanıcının kimliğini LTI başlatmasında gönderir ve FastComments onları Sakai hesaplarıyla oturum açmış şekilde tanımlar.

Rol eşlemesi:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (Administration Workspace yöneticisi) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai ile ilgili sorunlar**

- **Manage Tools içinde araç görünmüyor.** FastComments External Tools listesinde görünmüyorsa, Sakai yöneticisi araç kaydını açmalı (**Administration Workspace** > **External Tools** > **FastComments**) ve **Stealthed**'i `false` olarak ayarlamalıdır. Stealthed yapılan araçlar site bazlı Manage Tools seçicisinden gizlenir.
- **Paylaşılan oturumlu tarayıcılarda başlatmaların bozulması.** Sakai portal CSRF belirteci tarayıcı oturumuna bağlıdır. Bir öğrenci farklı sekmelerde iki Sakai sitesine aynı anda oturum açmışsa veya oturum süresi dolmuşsa, başlatma 403 döndürebilir. Çözüm: diğer Sakai sekmelerini kapatın, çıkış yapın, tekrar giriş yapın ve yeniden başlatın. Yöneticiler, bu durum küme genelinde oluyorsa `sakai.csrf.token.cache.ttl` değerini artırabilir.
- **Çerçeve (frame) gömme.** Yorum konusunun Lessons sayfası içinde kırpılmaması için `sakai.properties` içindeki `lti.frameheight` değerinin yeterince büyük (600 veya daha yüksek) olduğundan emin olun.

#### Schoology

Schoology Enterprise için iki kurulum senaryosu vardır. Araca eklemeden önce hangisinin geçerli olduğunu doğrulayın.

**1. İki kurulum senaryosu**

- **(a) Kurum düzeyinde kurulum.** Schoology Sistem Yöneticisi FastComments'i kuruluş düzeyinde kurdu ve tüm kurslara veya belirli kurs şablonlarına atadı. Eğitmenler kurulumu atlayıp doğrudan "Add Materials" bölümüne geçer.
- **(b) Eğitmenin kendi kendine yüklemesi.** Eğitmen, aracı tek bir kursa **Course Options** > **External Tools** > **Install LTI Apps** üzerinden kurar. Kendi kendine yükleme, önce Sistem Yöneticisinin FastComments uygulamasını kuruluş düzeyinde onaylamış olmasını gerektirir.

**2. FastComments'i kurs materyali olarak ekleme**

Kurs içinde:

1. Kursu açın ve **Materials**'a gidin.
2. **Add Materials** > **Add File/Link/External Tool**'a tıklayın.
3. **External Tool**'u seçin.
4. Kayıtlı araçlar listesinden **FastComments**'i seçin.
5. Bir **Name** belirleyin (öğrencilerin materyal listesinde gördüğü ad) ve isteğe bağlı bir **Description** ekleyin.
6. **Enable Grading**'i (not geri bildirimi) KAPALI bırakın. FastComments notları Schoology'ye raporlamaz, bu yüzden not geri bildirimi etkinleştirmek boş bir not defteri sütunu oluşturur.
7. **Submit**'e tıklayın.

Materyal şimdi kurs materyalleri listesinde görünür ve tıklanınca FastComments konusunu açar.

**3. Zengin Metin düzenleyici ile satır içi gömme**

Sistem Yöneticisi kayıt sırasında FastComments için Deep Linking yerleşimini etkinleştirdiyse, eğitmenler yorum konusunu herhangi bir Zengin Metin alanına (ödev talimatları, sayfa gövdeleri, tartışma yönlendirmeleri) gömebilir:

1. Hedef sayfadaki Zengin Metin düzenleyicisini açın.
2. Araç çubuğunda **External Tool** (yapboz parçası) simgesine tıklayın.
3. **FastComments**'i seçin.
4. Deep-linking iletişim kutusunda gömmeyi yapılandırın ve **Insert**'e tıklayın.
5. Sayfayı kaydedin.

Eğer External Tool düğmesi Zengin Metin düzenleyicisinde görünmüyorsa, bu tenant için Deep Linking devre dışı bırakılmış demektir. Aşağıdaki sorun giderme bölümüne bakın.

**4. Görünürlük ve bölüm atamaları**

Schoology, araç kullanılabilirliğini Kurs Seçenekleri üzerinden bölüm bazında sınırlar:

1. Kurstan **Course Options** > **External Tools**'a tıklayın.
2. Kurulu her LTI uygulaması için, uygulamanın kurstaki tüm bölümlere mi yoksa belirli bölümlere mi açık olacağını kontrol edersiniz.
3. FastComments'i belirli bölümlere sınırlamak için, aracı görmemesi gereken bölümlerin seçimlerini kaldırın.
4. Bölüm düzeyindeki erişim ayrıca hangi bölümlerin **Add Materials** > **External Tool** girişini görmesini de sınırlar.

**5. Öğrencilerin gördükleri**

Öğrenciler FastComments materyaline tıklar (veya satır içi gömülmüş alana kaydırır) ve dizili tartışmaya gelirler. SSO, Schoology LTI başlatması ile onların Schoology hesapları üzerinden otomatik olarak gerçekleştirilir.

Rol eşlemesi:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology ile ilgili sorunlar**

- **Sadece Enterprise.** Kişisel ve ücretsiz Schoology hesapları LTI 1.3 araçlarını yükleyemez. Tenantınız ücretsiz plandaysa, **External Tools** seçeneği Course Options içinde bulunmaz. FastComments kullanmak için Schoology Enterprise'a geçin.
- **Tenant varsayılanı olarak Deep Linking devre dışı.** Bazı Schoology tenant’ları kuruluş düzeyinde Deep Linking yerleştirmesini kısıtlar. Bu durumda eğitmenler sadece **Add Materials** > **External Tool** akışını görür ve Zengin Metin düzenleyicisinde External Tool düğmesini görmezler. Satır içi gömme etkinleştirmek için Sistem Yöneticisi **System Settings** > **Integration** > **LTI 1.3** > **FastComments** bölümüne gidip **Content Item / Deep Linking** yerleşimini etkinleştirip kaydeder.
- **Bölüm bazlı atama geçersiz kılma.** FastComments kurum düzeyinde atanmışsa ancak eğitmen **Add Materials** içinde göremiyorsa, kursun bölümü kuruluş düzeyindeki atamada hariç tutulmuştur. Sistem Yöneticisinden FastComments uygulaması atamasına bölümü eklemesini isteyin.
- **Materyal adı ile konu kimliği arasındaki fark.** Schoology'de materyalin adını değiştirmek yorum konusunu taşımaz. Konular LTI kaynak bağlantı kimliğine göre anahtarlanır, bu nedenle yeniden adlandırma aynı konuyu korur; materyali silip yeniden oluşturmak yeni, boş bir konu oluşturur.