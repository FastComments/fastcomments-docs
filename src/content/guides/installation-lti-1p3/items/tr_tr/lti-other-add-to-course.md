FastComments platforma kaydedildikten sonra, eğitmenler platformun standart harici araç iş akışlarını kullanarak bunu ders içeriğine ekler. Bu sayfa Sakai 23.x ve Schoology Enterprise için adımları kapsar.

#### Sakai

**1. Bir siteye FastComments ekleyin**

Site bakımcısı aracı site bazında etkinleştirir:

1. Siteyi açın ve sol gezinmede **Site Info** öğesine tıklayın.
2. **Manage Tools** öğesine tıklayın.
3. **External Tools** listesini aşağı kaydırın ve **FastComments** öğesini açın.
4. **Continue** öğesine tıklayın, araç listesini gözden geçirin, ardından **Finish** öğesine tıklayın.

FastComments artık sitede sol gezinme öğesi olarak görünür.

**2. Sol gezinme girişinin sırasını değiştirin**

**Site Info** > **Tool Order** bölümüne gidin. **FastComments** öğesini istenen konuma sürükleyin ve **Save** öğesine tıklayın. Bu ekranda gezinme etiketini yeniden adlandırabilir ve öğrencilere görünmesini gizleyebilirsiniz.

**3. Lessons sayfasına satır içi gömme**

FastComments'i bağımsız bir sol-nav aracı olarak değil de bir Lessons sayfasının içine doğrudan yerleştirmek için:

1. Sitede **Lessons** aracını açın.
2. **Add Content** > **Add External Tool** öğelerine tıklayın.
3. Listeden **FastComments** öğesini seçin.
4. FastComments kayıt sırasında Deep Linking duyurduysa, Sakai araç içerik seçicisini açar ve iş parçacığını seçebilir veya etiketleyebilirsiniz. Deep Linking duyurulmadıysa, Sakai varsayılan bir başlatma bağlantısı ekler.
5. Lessons öğesini kaydedin.

Her gömülü örneğin kendi kaynak bağlantısına göre sınırlandırılmış ayrı bir iş parçacığı olur.

**4. Öğrenci erişimi için izin ayarları**

Sakai harici araç başlatmalarını Realms üzerinden sınırlar. Öğrencilerin FastComments başlatabildiğini doğrulamak için:

1. Bir Sakai yöneticisi olarak oturum açın ve **Administration Workspace** > **Realms** öğesini açın.
2. İlgili realm'i açın (örneğin, `!site.template.course` veya belirli site realm'i).
3. `access` rolünün `lti.launch` etkinleştirildiğini ve **external.tools** grubundaki rol izinlerinin verildiğini onaylayın.
4. Realm'i kaydedin.

Site düzeyindeki geçersiz kılmalar için, bakımcı **Site Info** > **Tool Order** bölümünden rol başına araç görünürlüğünü FastComments'i gizleyerek veya göstererek ayarlayabilir.

**5. Öğrencilerin gördüğü şey**

Öğrenciler FastComments sol-nav öğesine tıklar (veya gömülü Lessons bloğuna kaydırır) ve doğrudan sıralı yorum görünümüne gelir. SSO otomatik çalışır: Sakai LTI başlatmasında kullanıcının kimliğini gönderir ve FastComments onları Sakai hesapları ile oturum açtırır.

Rol eşlemesi:

- Sakai `Instructor` -> FastComments moderatörü
- Sakai `Admin` (Administration Workspace içindeki admin) -> FastComments yönetici
- Sakai `Student` / `access` -> FastComments yorumcu

**6. Sakai tuzakları**

- **Manage Tools içinde araç görünmüyor.** Eğer FastComments External Tools listesinde görünmüyorsa, Sakai yöneticisi araç kayıt defterini açmalı (**Administration Workspace** > **External Tools** > **FastComments**) ve **Stealthed** değerini `false` olarak ayarlamalıdır. Stealthed araçlar site bazındaki Manage Tools seçicisinden gizlenir.
- **Paylaşılan oturumlu tarayıcılarda başlatmaların kırılması.** Sakai'nın portal CSRF belirteci tarayıcı oturumuna bağlıdır. Eğer bir öğrenci farklı sekmelerde iki Sakai sitesine kayıtlıysa veya oturum süresi dolmuşsa, başlatma 403 döndürebilir. Çözüm: diğer Sakai sekmelerini kapatın, çıkış yapın, tekrar giriş yapın ve yeniden başlatın. Yöneticiler bunun küme genelinde olması durumunda `sakai.csrf.token.cache.ttl` değerini yükseltebilir.
- **Çerçeve içinde gömme.** Yorum iş parçacığının Lessons sayfası içinde kırpılmaması için `sakai.properties` içindeki `lti.frameheight` değerinin yeterince büyük (600 veya üzerinde) olduğunu doğrulayın.

#### Schoology

Schoology Enterprise'ın iki kurulum senaryosu vardır. Aracı bir derse eklemeden önce hangi senaryonun geçerli olduğunu doğrulayın.

**1. İki kurulum senaryosu**

- **(a) Kurumsal düzey kurulum.** Schoology Sistem Yöneticisi FastComments'i kuruluş düzeyinde yüklemiş ve tüm derslere veya belirli ders şablonlarına atamıştır. Eğitmenler kurulumu atlar ve doğrudan "Add Materials" işlemine gider.
- **(b) Eğitmen tarafından kendi kurulum.** Eğitmen aracı tek bir derse **Course Options** > **External Tools** > **Install LTI Apps** üzerinden yükler. Kendi kurulum, Sistem Yöneticisinin önce FastComments uygulamasını organizasyon düzeyinde onaylamasını gerektirir.

**2. FastComments'i ders materyali olarak ekleyin**

Ders içinde:

1. Dersi açın ve **Materials** bölümüne gidin.
2. **Add Materials** > **Add File/Link/External Tool** öğesine tıklayın.
3. **External Tool** seçeneğini seçin.
4. Kayıtlı araçlar listesinden **FastComments** öğesini seçin.
5. Bir **Name** belirleyin (öğrencilerin materyaller listesinde gördükleri isim budur) ve isteğe bağlı bir **Description** girin.
6. **Enable Grading** (grade passback) seçeneğini KAPALI bırakın. FastComments notları Schoology'ye geri raporlamaz, bu yüzden not geri bildirimini etkinleştirmek boş bir not defteri sütunu oluşturur.
7. **Submit** öğesine tıklayın.

Materyal artık ders materyalleri listesinde görünür ve tıklanınca FastComments iş parçacığını açar.

**3. Zengin Metin düzenleyici aracılığıyla satır içi yerleştirme**

Sistem Yöneticisi kayıt sırasında FastComments için Deep Linking yerleştirmesini etkinleştirdiyse, eğitmenler herhangi bir Zengin Metin alanının (ödev talimatları, sayfa gövdeleri, tartışma yönlendirmeleri) içine yorum iş parçacığını yerleştirebilir:

1. Hedef sayfadaki Zengin Metin düzenleyicisini açın.
2. Araç çubuğundaki **External Tool** (yapboz parçası) simgesine tıklayın.
3. **FastComments** öğesini seçin.
4. Derin bağlama iletişim kutusunda gömmeyi yapılandırın ve **Insert** öğesine tıklayın.
5. Sayfayı kaydedin.

Eğer Zengin Metin düzenleyicisinde External Tool düğmesi görünmüyorsa, bu araç için tenant üzerinde Deep Linking devre dışıdır. Aşağıdaki tuzaklara bakın.

**4. Görünürlük ve bölüm atamaları**

Schoology, Course Options aracılığıyla aracın kullanılabilirliğini bölüm bazında sınırlar:

1. Dersten **Course Options** > **External Tools** öğesine tıklayın.
2. Yüklü her LTI uygulaması için, uygulamanın dersteki tüm bölümlere mi yoksa belirli bölümlere mi açık olduğunu kontrol edersiniz.
3. FastComments'i belirli bölümlere sınırlamak için aracı görmemesi gereken bölümlerin işaretini kaldırın.
4. Bölüm düzeyindeki erişim ayrıca hangi bölümlerin **Add Materials** > **External Tool** girdisini göreceğini belirler.

**5. Öğrencilerin gördüğü şey**

Öğrenciler FastComments materyaline tıklar (veya satır içi gömülene kaydırır) ve sıralı tartışma içinde açılırlar. SSO, Schoology LTI başlatması aracılığıyla ve kendi Schoology hesaplarıyla otomatik olarak gerçekleşir.

Rol eşlemesi:

- Schoology `Administrator` -> FastComments yönetici
- Schoology `Instructor` -> FastComments moderatörü
- Schoology `Student` -> FastComments yorumcu

**6. Schoology tuzakları**

- **Sadece Kurumsal.** Kişisel ve ücretsiz Schoology hesapları LTI 1.3 araçlarını kuramaz. Eğer tenant'ınız ücretsiz düzeydeyse, Course Options içinde **External Tools** seçeneği yoktur. FastComments kullanmak için Schoology Enterprise'a yükseltin.
- **Tenant varsayılanı olarak Deep Linking devre dışı.** Bazı Schoology tenant'ları Deep Linking yerleştirmesini organizasyon düzeyinde kısıtlar. Bu durumda eğitmenler yalnızca **Add Materials** > **External Tool** akışını görür; Zengin Metin düzenleyicisinde External Tool düğmesi görünmez. Satır içi gömme etkinleştirmek için Sistem Yöneticisi **System Settings** > **Integration** > **LTI 1.3** > **FastComments** yolunu izler ve **Content Item / Deep Linking** yerleştirmesini etkinleştirip kaydeder.
- **Bölüm bazında atama geçersiz kılma.** FastComments kurumsal düzeyde atanmışsa fakat eğitmen **Add Materials** içinde göremiyorsa, dersin bölümü organizasyon düzeyindeki atamada hariç tutulmuştur. Sistem Yöneticisinden bölümü FastComments uygulaması atamasına eklemesini isteyin.
- **Materyal adı ile iş parçacığı kimliği farkı.** Schoology'de materyalin adını yeniden adlandırmak yorum iş parçacığını taşımaz. İş parçacıkları LTI kaynak bağlantı kimliğine göre anahtarlanır, bu yüzden yeniden adlandırma aynı iş parçacığını korur; materyali silip yeniden oluşturmak yeni ve boş bir iş parçacığı oluşturur.