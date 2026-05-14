Bu kılavuz, site yöneticisi aracı kaydettikten ve etkinlik seçicide gösterilecek şekilde ayarladıktan sonra Moodle 4.x dersine FastComments eklemeyi kapsar. Eğer FastComments henüz kaydedilmediyse, önce Moodle kayıt kılavuzuna bakın.

#### Dersi Düzenleme Modunda Açın

1. Ders için Moodle'a bir Editing Teacher (veya daha yüksek) olarak giriş yapın.
2. Dersi açın.
3. Ders başlığının sağ üst köşesindeki anahtarı kullanarak **Edit mode**'u açın.

Moodle 4.x, 3.x'in kullandığı eski "Add an activity or resource" açılır menüsünün yerini tam ekran bir etkinlik seçici iletişim kutusuna bıraktı. Moodle 4.5 aynı seçiciyi korur ancak üstte yıldızlanmış/favoriler satırı ekler; böylece FastComments'i sabitlemek, ileride erişimi hızlandırır.

#### FastComments Etkinliğini Ekle

1. Tartışmanın ait olduğu ders bölümüne (konu veya hafta) gidin.
2. O bölümün altındaki **Add an activity or resource**'a tıklayın.
3. Seçici iletişim kutusunda **FastComments**'i seçin. Görmüyorsanız, aşağıdaki uyarılar bölümüne atlayın.

Etkinlik ayarları formu açılır. Önemli alanlar:

- **Activity name** (required). Ders sayfasında ve not defterinde gösterilir. Örnek: `Week 3 Discussion`.
- **Activity description**. Yorum dizisinin üstünde görüntülenen isteğe bağlı giriş metni.
- **Show description on course page**. Açıklamanın etkinliğe tıklamadan görünmesini istiyorsanız işaretleyin.
- **Preconfigured tool**. `FastComments` olarak ayarlı (seçiciden başlatıldığında otomatik seçilir). Değiştirmeyin.
- **Launch container**. **New window** olarak ayarlayın. Bazı Moodle dağıtımlarında "Same window" seçeneğinin neden bozulduğunu görmek için uyarılar bölümüne bakın.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Boş bırakın. Dinamik Kayıt bunları site düzeyinde ele aldı.

Sayfanın altına kaydırın ve **Save and return to course**'a tıklayın (veya etkinliği hemen açmak için **Save and display**).

Etkinlik, FastComments simgesiyle bölümde bir satır olarak görünür. Öğrenciler yorum dizisini açmak için satıra tıklar.

#### Editörde FastComments'i Satır İçi Gömme

Bir Page, Book bölümü, Lesson veya Atto ya da TinyMCE editörünü kullanan herhangi bir kaynak içinde bir dizi oluşturmak için:

1. Kaynağı düzenleme modunda açın.
2. Dizinin görünmesini istediğiniz yere imleci yerleştirin.
3. Editör araç çubuğunda **LTI** / **External tool** düğmesine tıklayın. Atto'da "Insert LTI Advantage content" olarak etiketlenmiştir. TinyMCE'de (Moodle 4.3+ varsayılan) **More** menüsünün altında **External tools** olarak bulunur.
4. Araç listesinden **FastComments**'i seçin.
5. FastComments derin bağlantı (deep-linking) seçici açar. Dizinin başlığını onaylayın ve **Embed**'e tıklayın.
6. Editör bir LTI yer tutucu bloğu ekler. Kaynağı kaydedin.

Her gömülü örnek, deep-link content item ID ile anahtarlanan ayrı bir dizidir; bu yüzden bir Sayfada üç FastComments gömüsü varsa üç bağımsız dizi olur.

#### Erişimi Kısıtlama ve Grup Ayarları

FastComments etkinlikleri için standart Moodle etkinlik ayarları uygulanır:

- **Common module settings** > **Group mode**. Bunu **Separate groups** veya **Visible groups** olarak ayarlamak FastComments'i otomatik olarak grup başına ayrı dizilere bölmez. Moodle'ın grup modu yalnızca not defterini ve üye listesini filtreler. Her grup için ayrı bir dizi çalıştırmak istiyorsanız, grup başına bir FastComments etkinliği ekleyin ve her birini kapsamlamak için **Restrict access** kullanın.
- **Restrict access** > **Add restriction**. Standart Moodle koşullarını destekler: **Date**, **Grade**, **Group**, **Grouping**, **User profile** ve iç içe geçmiş kısıtlama setleri. Bir FastComments etkinliğini tek bir gruba kilitlemek için **Group** kullanın.
- **Activity completion**. Tamamlama takibi istiyorsanız **Students must view this activity to complete it** olarak ayarlayın. FastComments şu anda başlatmanın ötesinde Moodle'a bir tamamlama etkinliği raporlamaz.

#### Rol Eşlemesi

FastComments, Moodle'ın her başlatmada gönderdiği LTI `roles` iddiasını okur ve şu şekilde eşler:

- Moodle **Manager** veya **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** veya **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> salt okunur

Admins herhangi bir yorumu silebilir, kullanıcıları yasaklayabilir ve dizi ayarlarını düzenleyebilir. Moderatlar başlattıkları dizinin içindeki yorumları silebilir ve onaylayabilir. Özelleştirilmiş Moodle rolleri, klonlandıkları arketipin eşlemesini miras alır.

#### Öğrencilerin Gördükleri

Öğrenciler FastComments etkinliğine tıklar (veya bir Page ya da Book içindeki gömülü bloğa kaydırır). Moodle kimliklerini LTI başlatması aracılığıyla FastComments'e gönderir:

- Giriş ekranı yok. FastComments, onları Moodle hesabını kullanarak oturum açtırır.
- Görünen adları, e-posta ve avatarları Moodle'dan gelir.
- Dizi `(Moodle site, course, resource link ID)` kapsamına alınır, bu nedenle aynı etkinliğin başka bir derse kopyalanması yeni bir dizi oluşturur.
- İletili yanıtlar, oy verme ve bildirimler, bağımsız bir FastComments dizisiyle aynı şekilde çalışır.

#### Moodle Uyarıları

**FastComments etkinlik seçicide yok.** Site yöneticisi aracı kaydetti ancak **Tool configuration usage**'ı **Show in activity chooser and as a preconfigured tool** olarak ayarlamadı. Bunu düzeltmek için **Site yönetimi** > **Eklentiler** > **Etkinlik modülleri** > **Harici araç** > **Araçları yönet** > FastComments kutucuğundaki dişli simgesine gidin.

**"Same window" seçiliyken başlatma başarısız oluyor veya boş bir çerçeve gösteriyor.** Moodle'ın oturum çerezleri varsayılan olarak `SameSite=Lax` kullanır ve bazı tarayıcılar LTI 1.3'ün FastComments'ten geri dönüş için kullandığı çapraz site POST sırasında bu çerezleri kaldırır. Etkinlikte **Launch container**'ı **New window** olarak ayarlayın. Bu, bir Page veya Book içine gömülü FastComments için katı bir gereksinimdir; çünkü editöre gömülü başlatma yolu her zaman yeni bir pencere açar.

**`iss` iddiası Moodle site URL'si, tenant ID değil.** FastComments, LTI issuer olarak Moodle site URL'sini (`wwwroot` yapılandırma değeri) kullanır. Moodle örneğiniz yeni bir domaine taşınırsa veya `wwwroot` değerini değiştirirseniz, mevcut FastComments dizileri eski issuer ile bağlı kalır ve yeni başlatmalarla eşleşmez. Gerekirse aracı yeni URL'ye karşı yeniden kaydedin ve dizileri FastComments yöneticisi aracılığıyla taşıyın.

**Etkinlik yedekleme ve geri yükleme.** Bir kursu yedekleyip yeni bir kursa geri yüklemek yeni resource link ID'leri oluşturur, bu nedenle geri yüklenen FastComments etkinlikleri boş dizilerle başlar. Orijinal kurs orijinal dizileri muhafaza eder. Bu amaçlanmış bir davranıştır, hata değildir.

**Moodle 4.5 TinyMCE varsayılan.** Moodle 4.5, yeni kurulumlar için varsayılan editör olarak TinyMCE ile gelir. External tool düğmesinin konumu ana araç çubuğu yerine **More** (`...`) menüsünün altındadır. 4.1'den yükseltilen daha eski siteler, bir yönetici varsayılanı değiştirmedikçe Atto'yu korur.