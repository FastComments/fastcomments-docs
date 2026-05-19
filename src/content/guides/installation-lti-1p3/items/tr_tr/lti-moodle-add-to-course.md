Bu kılavuz, bir site yöneticisinin aracı kaydedip etkinlik seçicisinde göstermesinin ardından FastComments'in bir Moodle 4.x dersine nasıl ekleneceğini açıklar. FastComments henüz kaydedilmediyse önce Moodle kayıt kılavuzuna bakın.

#### Dersi Düzenleme Modunda Açın

1. Ders için Düzenleme Öğretmeni (veya daha üstü) olarak Moodle'a giriş yapın.  
2. Dersi açın.  
3. Ders başlığının sağ üst köşesindeki anahtar ile **Düzenleme modunu** açın.

Moodle 4.x, 3.x'te kullanılan eski "Add an activity or resource" açılır menüsünü tam ekran etkinlik seçici dialoguyla değiştirdi. Moodle 4.5 aynı seçiciyi koruyor ancak üstte yıldızlı/favoriler satırı ekliyor; FastComments'i bir kez sabitlemek, sonraki bölümlerde daha hızlı ulaşmanızı sağlar.

#### FastComments Etkinliğini Ekle

1. Tartışmanın ait olduğu ders bölümüne (konu veya hafta) kaydırın.  
2. O bölümün altındaki **Add an activity or resource** öğesine tıklayın.  
3. Seçici dialogunda **FastComments**'i seçin. Görmüyorsanız aşağıdaki gotchas bölümüne atlayın.

Etkinlik ayarları formu açılır. Önemli alanlar:

- **Etkinlik adı** (gerekli). Ders sayfasında ve not defterinde gösterilir. Örnek: `Week 3 Discussion`.  
- **Etkinlik açıklaması**. Yorum dizisinin üstünde görüntülenen isteğe bağlı giriş metni.  
- **Açıklamayı ders sayfasında göster**. Açıklamanın etkinliğe tıklamadan görünmesini istiyorsanız bunu işaretleyin.  
- **Ön yapılandırılmış araç**. `FastComments` olarak ayarlı (seçiciden başlatıldığında otomatik seçilir). Değiştirmeyin.  
- **Başlatma kapsayıcısı**. **Yeni pencerede** olarak ayarlayın. Bazı Moodle dağıtımlarında "Aynı pencerede" seçeneğinin neden bozulduğunu görmek için gotchas bölümüne bakın.  
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Boş bırakın. Dinamik Kayıt bunları site düzeyinde yönetir.

Sayfanın altına kaydırın ve **Kaydet ve derse dön** (veya etkinliği hemen açmak için **Kaydet ve göster**) öğesine tıklayın.

Etkinlik, ilgili bölümde FastComments simgesiyle bir satır olarak görünür. Öğrenciler yorum dizisini açmak için bu satıra tıklar.

#### Düzenleyiciyle Satır İçi Olarak FastComments Gömme

Atto veya TinyMCE düzenleyicisini kullanan bir Page, Book bölümü, Lesson veya diğer herhangi bir kaynaktaki bir konu için:

1. Kaynağı düzenleme modunda açın.  
2. İmleci dizinin görünmesini istediğiniz yere yerleştirin.  
3. Düzenleyici araç çubuğunda **LTI** / **External tool** düğmesine tıklayın. Atto'da "Insert LTI Advantage content" olarak etiketlenmiştir. TinyMCE'de (Moodle 4.3+'te varsayılan) **More** menüsünün altında **External tools** olarak bulunur.  
4. Araç listesinden **FastComments**'i seçin.  
5. FastComments derin bağlantı seçici açar. Konu başlığını onaylayın ve **Embed**'e tıklayın.  
6. Düzenleyici bir LTI yer tutucu bloğu ekler. Kaynağı kaydedin.

Her gömülü örnek, deep-link content item ID ile anahtarlanmış ayrı bir konu dizisidir, bu nedenle üç FastComments gömülü ögesi olan bir Page üç bağımsız konu dizisi alır.

#### Erişimi Kısıtlama ve Grup Ayarları

FastComments etkinliklerine Moodle'ın standart etkinlik ayarları uygulanır:

- **Common module settings** > **Group mode**. Bunu **Separate groups** veya **Visible groups** olarak ayarlamak, FastComments'i otomatik olarak grup başına ayrı konu dizilerine bölmez. Moodle'ın grup modu yalnızca not defterini ve üye listesini filtreler. Grup başına ayrı bir konu dizisi çalıştırmak için her grup için bir FastComments etkinliği ekleyin ve her birini kapsamlamak için **Restrict access** kullanın.  
- **Restrict access** > **Add restriction**. Standart Moodle koşullarını destekler: **Date**, **Grade**, **Group**, **Grouping**, **User profile** ve iç içe kısıtlama setleri. Bir FastComments etkinliğini tek bir grupla kilitlemek için **Group**'u kullanın.  
- **Activity completion**. Tamamlanma takibi istiyorsanız **Students must view this activity to complete it** olarak ayarlayın. FastComments şu anda başlatma dışında Moodle'a bir tamamlanma olayı bildirmez.

#### Rol Eşlemesi

FastComments, Moodle'ın her başlatmada gönderdiği LTI `roles` bildirgesini okur ve şu şekilde eşler:

- Moodle **Manager** veya **Site administrator** -> FastComments **admin**  
- Moodle **Editing teacher** veya **Non-editing teacher** -> FastComments **moderator**  
- Moodle **Student** -> FastComments **commenter**  
- Moodle **Guest** -> salt okunur

Yöneticiler herhangi bir yorumu silebilir, kullanıcıları yasaklayabilir ve konu ayarlarını düzenleyebilir. Moderatörler, girdikleri konu dizisi içinde yorumları silebilir ve onaylayabilir. Özel Moodle rolleri, klonlandıkları örnek rolün eşlemesini miras alır.

#### Öğrencilerin Gördükleri

Öğrenciler FastComments etkinliğine tıklar (veya bir Page ya da Book içindeki gömülü bloğa kaydırır). Moodle, başlatma sırasında kimliklerini FastComments'e LTI aracılığıyla gönderir:

- Giriş ekranı yok. FastComments onları Moodle hesabı kullanarak oturum açtırır.  
- Görünen adları, e‑posta adresleri ve avatarları Moodle'dan gelir.  
- Konu dizisi `(Moodle site, course, resource link ID)` ile sınırlandırılmıştır, bu nedenle aynı etkinliğin başka bir derse kopyalanması yeni bir konu dizisi oluşturur.  
- Konuya bağlı yanıtlar, oylama ve bildirimler ayrı bir FastComments konu dizisindeki gibi çalışır.

#### Genel Erişimi Kısıtlayın (Önerilir)

Varsayılan olarak FastComments yorum verileri herkese açık okunabilir. Bir konu dizisinin URL'sini veya API uç noktasını tahmin edebilen herhangi biri, Moodle dışında bile yorumları görüntüleyebilir. Ders tartışmaları için neredeyse kesinlikle görüntülemeyi yalnızca kayıtlı öğrencilere sınırlamak istersiniz.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Widget özelleştirme sayfanızı</a> açın ve **Require SSO To View Comments** etkin olan bir kural oluşturun, ardından güvenlik düzeyini **Secure SSO** olarak ayarlayın; böylece konu dizileri yalnızca imzalı LTI başlatması aracılığıyla yüklenebilir.

Tam adım adım kılavuz ve kuralın tek bir alan adı veya sayfaya nasıl kısıtlanacağı dahil ayrıntılar için [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) bölümüne bakın.

#### Moodle Uyarıları

**Etkinlik seçicisinde FastComments yok.** Site yöneticisi aracı kaydetmiş ancak **Tool configuration usage**'ı **Show in activity chooser and as a preconfigured tool** olarak ayarlamamış olabilir. Bunu düzeltmek için şu yolu izleyin: **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > FastComments kutucuğundaki dişli simgesi.

**"Aynı pencerede" olarak ayarlandığında başlatma başarısız oluyor veya boş bir çerçeve gösteriyor.** Moodle'ın oturum çerezleri varsayılan olarak `SameSite=Lax` kullanır ve bazı tarayıcılar LTI 1.3'ün FastComments'ten geri dönmek için kullandığı çapraz site POST sırasında bu çerezleri kaldırır. Etkinlikte **Başlatma kapsayıcısı**'nı **Yeni pencerede** olarak ayarlayın. Bu, editöre gömülü FastComments için zorunlu bir gereksinimdir, çünkü düzenleyiciye gömülü başlatma yolu her zaman yeni bir pencere açar.

**`iss` bildirgesi Moodle site URL'sidir, tenant ID değil.** FastComments, LTI issuer olarak Moodle site URL'sini (`wwwroot` konfigürasyon değeri) kullanır. Moodle örneğiniz yeni bir domaine taşınırsa veya `wwwroot`'u değiştirirseniz, mevcut FastComments konu dizileri eski issuer'a bağlı kalır ve yeni başlatmalarla eşleşmez. Aracı yeni URL için yeniden kaydedin ve gerekirse konu dizilerini FastComments yöneticisi aracılığıyla taşıyın.

**Etkinlik yedekleme ve geri yükleme.** Bir dersi yedekleyip yeni bir derse geri yüklemek yeni resource link ID'leri oluşturur; bu nedenle geri yüklenen FastComments etkinlikleri boş konu dizileriyle başlar. Orijinal ders orijinal konu dizilerini korur. Bu, kasıtlı bir davranıştır, bir hata değildir.

**Moodle 4.5 TinyMCE varsayılanı.** Moodle 4.5, yeni kurulumlar için varsayılan düzenleyici olarak TinyMCE ile gelir. External tool düğmesinin yeri ana araç çubuğu yerine **More** (`...`) menüsünün altındadır. 4.1'den yükseltilen daha eski siteler, bir yönetici varsayılanı değiştirmedikçe Atto'yu korur.