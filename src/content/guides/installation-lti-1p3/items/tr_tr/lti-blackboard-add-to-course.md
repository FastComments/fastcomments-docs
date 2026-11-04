Bir yönetici FastComments'i bir LTI 1.3 Advantage aracı olarak kaydettikten ve kurum politikalarını onayladıktan sonra, eğitmenler onu standart Blackboard yerleştirme noktaları aracılığıyla derslere ekler. Kesin adımlar Ultra Course View ile Original Course View arasında farklılık gösterir, bu nedenle her ikisi de aşağıda ele alınmıştır.

#### Ultra Course View

Ultra Course View, 2026 itibarıyla Blackboard Learn SaaS'ta varsayılan olarak gelir.

1. Dersi açın ve **Course Content** sayfasına gidin.
2. Yorum dizisinin ana hatlarda nerede görünmesini istiyorsanız imleci oraya getirin veya dokunun ve mor **+** (Add content) düğmesine tıklayın.
3. **Content Market** seçeneğini seçin. Content Market paneli, kurumunuz için onaylanmış her LTI aracını ve Building Block yerleştirmesini listeler.
4. **FastComments** kutucuğunu bulun ve tıklayın. Blackboard, **+** menüsünü açtığınız konumda bir içerik öğesi oluşturur.
5. Öğe, kişisel varsayılanı **Hide from students** kapalı olan eğitmenler için ana hatlarda varsayılan olarak "Visible to students" girişi olarak yerleşir. Varsayılanınız **Hidden** ise, öğe gizli olarak oluşturulur ve hazır olduğunuzda öğe satırındaki görünürlük seçicisini değiştirirsiniz.
6. Öğeyi yeniden adlandırmak için ana hatlardaki başlığa tıklayın ve yeni bir etiket yazın. Öğrencilerin ana hatlarda gördüğü başlık, FastComments dizi tanımlayıcısından bağımsızdır, bu nedenle yeniden adlandırma her zaman güvenlidir.

**Content Market** seçeneğini görmüyorsanız, kurumunuz yerleştirmeyi gizlemiştir. Aynı seçiciye, aynı **+** menüsünde **LTI Tools** grubu altındaki **More tools** üzerinden de ulaşabilirsiniz.

#### Original Course View

Original Course View, Learn SaaS'ta hâlâ desteklenmektedir ve Q4 2024 CU sürüm hattındaki kendi kendine barındırılan Learn 9.1 siteleri için birincil deneyim olmaya devam etmektedir.

1. Dersi açın ve bir **Content Area** alanına girin (örneğin, ders menüsündeki varsayılan **Information** veya **Content** alanı).
2. Sayfanın sağ üstündeki anahtarla **Edit Mode** modunu açın.
3. Eylem çubuğunda **Build Content** seçeneğine tıklayın.
4. **Learning Tools** alt menüsünde **FastComments** seçeneğine tıklayın. Learning Tools alt menüsü, bir yönetici aracı kaydettikten sonra LTI 1.3 araç yerleştirmelerinden doldurulur. Görmüyorsanız, aşağıdaki gotchas bölümüne bakın.
5. **Create FastComments** formunda şunları ayarlayın:
   - **Name**: öğrencilerin içerik alanında gördüğü etiket.
   - **Description**: gömülü dizinin üzerinde gösterilen isteğe bağlı metin.
   - **Permit Users to View this Content**: Evet/Hayır kullanılabilirlik anahtarı.
   - **Track Number of Views**: Blackboard'un öğe başına görüntüleme istatistiklerini istiyorsanız etkinleştirin. FastComments kendi analitiğini bağımsız olarak yürütür.
   - **Date and Time Restrictions**: isteğe bağlı **Display After** / **Display Until** zaman aralıkları.
6. Gönderin. Araç, içerik alanında tıklanabilir bir öğe olarak görünür.

#### Bir Öğenin veya Belgenin İçine Gömme

Her iki ders görünümünde de eğitmenler, Content Editor'ın LTI Advantage düğmesi aracılığıyla FastComments'i bir Item, Document veya herhangi bir zengin metin alanının gövdesinin içine gömer.

Ultra Course View:

1. Bir **Document** oluşturun veya düzenleyin.
2. Dizinin görünmesini istediğiniz belge gövdesinin içinde **Add content** seçeneğine tıklayın.
3. Düzenleyici araç çubuğunda **Insert content** menüsünü açın ve **Content Market** (LTI Advantage / Deep Linking giriş noktası) seçeneğine tıklayın.
4. **FastComments** seçin. FastComments bir deep-link yükü döndürür ve Blackboard, belge gövdesine imleç konumuna gömülü bir blok ekler.
5. Belgeyi kaydedin. Öğrenciler, diziyi kaydırırken geçtiklerinde satır içinde işlenmiş olarak görürler.

Original Course View:

1. Zengin metin gövdesine sahip herhangi bir öğeyi düzenleyin.
2. Content Editor araç çubuğunda **Add Content** artı simgesine tıklayın ve **Content Market** seçeneğini seçin (eski Q4 2024 CU'larda **Add Content from External Tool** olarak etiketlenir).
3. **FastComments** seçin. Düzenleyici, deep-link'lenmiş kaynağa referans veren bir yer tutucu blok ekler.
4. Öğeyi gönderin.

Her deep-link gömme, kendi FastComments dizisini üretir; dolayısıyla iki gömülü FastComments bloğu içeren bir Item, iki bağımsız yorum akışına sahiptir.

#### Görünürlük, Release Conditions ve Grup Kısıtlamaları

FastComments içerik öğeleri, üzerlerine uygulanan erişim denetimi kuralları açısından diğer tüm Blackboard içerik öğeleri gibi davranır.

- Ultra: satırdaki görünürlük seçicisine tıklayın (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability; tarih/saat aralıklarını, gradebook öğelerine karşı performans kurallarını ve ders gruplarına karşı üye kurallarını destekler.
- Original: öğenin bağlam menüsünü açın ve aracı tarih, üyelik, not veya inceleme durumuna göre kısıtlamak için **Adaptive Release** veya **Adaptive Release: Advanced** seçeneğini seçin. Öğe üzerinde belirli ders gruplarıyla sınırlamak için **Set Group Availability** seçeneğini kullanın.

FastComments, Blackboard'un kapısının neye karar verdiğini takip eder. Blackboard bir öğrenci için öğeyi gizlerse, LTI lansmanı o öğrenci için hiç gerçekleşmez ve moderatör görünümünde görünmezler.

#### Gradebook Davranışı

FastComments, LTI Advantage Assignment and Grade Services üzerinden not geri bildirimi yapmaz. FastComments içerik öğeleri için otomatik olarak bir not sütunu oluşturulmaz.

Blackboard kiracınız, notlama meta verilerinden bağımsız olarak her yeni içerik öğesi için otomatik olarak bir gradebook sütunu oluşturacak şekilde yapılandırılmışsa, yine de boş bir sütun görünür. Gizlemek için:

- Ultra: **Gradebook** bölümünü açın, sütun başlığına tıklayın, **Edit** seçeneğini seçin ve **Show to students** ile **Include in calculations** seçeneklerini kapatın. Ya da kurumunuz notlanmamış öğeler için sütun silmeye izin veriyorsa **Delete** seçeneğini kullanın.
- Original: **Grade Center** bölümünü açın, sütunun chevron'ına tıklayın, **Hide from Users (on/off)** seçeneğini seçin ve isteğe bağlı olarak **Column Organization** altında **Hide from Instructor View** seçeneğini seçin.

#### Öğrencilerin Gördükleri

Bir öğrenci FastComments öğesini açtığında veya gömülü bir bloğa kaydırdığında:

1. Blackboard, FastComments'e LTI 1.3 mesajını başlatır. Öğrenci, bir giriş formu görmeden Blackboard kimliği (ad, e-posta, avatar, rol) ile SSO üzerinden oturum açar.
2. Yorum dizisi iframe içinde işlenir. Dallanma, yanıtlar, bahsetmeler ve tepkiler, FastComments'te yapılandırılan yorum widget'ı ayarlarına göre kullanılabilir.
3. Yorumları Blackboard hesaplarına atfedilir. Öğrenci daha sonra Blackboard'da adını veya fotoğrafını düzenlerse, bir sonraki lansman FastComments profilini günceller.

Blackboard'dan FastComments'e rol eşlemesi:

- **System Administrator** ve **Course Builder**, FastComments **admin** rolüne eşlenir.
- **Instructor** ve **Teaching Assistant**, FastComments **moderator** rolüne eşlenir.
- **Student**, **Guest** ve **Observer**, FastComments **commenter** rolüne eşlenir.

Moderatörler, dizideki her yorum üzerinde satır içinde moderasyon kontrollerini (sabitleme, gizleme, yasaklama, silme) görürler.

#### Genel Erişimi Kilitleme (Önerilir)

Varsayılan olarak, FastComments yorum verileri herkese açık şekilde okunabilir. Bir dizinin URL'sini veya API uç noktasını tahmin edebilen herkes, Blackboard dışında bile yorumlarını görüntüleyebilir. Ders tartışmaları için neredeyse kesinlikle görüntülemeyi yalnızca kayıtlı öğrencilerle sınırlamak istersiniz.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget özelleştirme sayfanızı</a> açın ve **Require SSO To View Comments** etkinleştirilmiş bir kural oluşturun, ardından güvenlik düzeyini **Secure SSO** olarak ayarlayın; böylece diziler yalnızca imzalı LTI lansmanı üzerinden yüklenebilir.

Kuralın tek bir alan adına veya sayfaya nasıl kapsanacağı dahil, tam rehber için [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) sayfasına bakın.

#### Dizi Kapsamlandırma

FastComments her diziyi **(Blackboard host, course ID, resource link ID)** ile kapsar. Aynı derste iki FastComments öğesi iki dizi üretir. Aynı öğe iki ders kabuğuna kopyalandığında (örneğin, ders kopyalama yoluyla) iki dizi üretilir, çünkü Blackboard kopyalama sırasında yeni bir resource link ID verir. Ders kopyaları arasında paylaşılan bir diziyi korumak için, kopyalamayı başlatmadan önce FastComments'te yapılandırılmış açık bir thread URN ile Deep Linking kullanın.

#### Blackboard'a Özgü Gotchas

**FastComments kutucuğu Build Content menüsünde (Original) veya Content Market'te (Ultra) eksik.** Yönetici aracı onayladı ancak ilgili yerleştirmeyi engelleyen bir kurum politikası bıraktı. **Administrator Panel** > **Integrations** > **LTI Tool Providers** bölümüne gidin, FastComments girişini düzenleyin ve hem **Course Content Tool** (Original) hem de **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) yerleştirmelerinin etkin olduğunu doğrulayın. Kaydedin ve ders sayfasını yenileyin.

**Lansmanda "Tool not configured for this context" veya "Tool is not deployed" hatası.** Dinamik kayıt sırasında kaydedilen dağıtım kapsamı, dersin ait olduğu kurum bağlamıyla eşleşmiyor. Blackboard'un araç sağlayıcı girişinde, **Deployment ID** değerinin FastComments'in bu kiracı için LTI 1.3 Configuration sayfasında gösterdiğiyle eşleştiğini doğrulayın. Farklılıkları varsa, yerleştirmeyi silin ve yeni bir kayıt URL'sinden dinamik kaydı yeniden çalıştırın (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alın</a>).

**Iframe yüksekliği sabit görünüyor veya içerik kesiliyor.** Bazı Blackboard kiracıları, varsayılan LTI iframe-yeniden boyutlandırma postMessage'ını engelleyen katı bir Content Security Policy ile gelir. FastComments, uyumluluğu en üst düzeye çıkarmak için hem Canvas tarzı `lti.frameResize` mesajını hem de IMS spec-form `org.imsglobal.lti.frameResize` mesajını yayar, ancak kiracı düzeyinde bir CSP geçersiz kılması ebeveyn dinleyiciyi engeller. Yöneticinizden `*.fastcomments.com` adresinin LTI araç izin listesinde olduğunu ve hiçbir özel CSP başlığının postMessage olaylarını ayıklamadığını doğrulamasını isteyin. Bundan sonra yeniden boyutlandırma, ek yapılandırma olmadan çalışır.

**Ders kopyalama dizileri çoğaltır.** Blackboard ders kopyalama, LTI yerleştirmeleri için yeni resource link ID'leri verir, bu nedenle kopyalanan dersler boş dizilerle başlar. Bu beklenen bir durumdur. Kopyalanan dersin orijinal diziyi devralmasını gerekiyorsa, kopyalamadan önce açık bir thread URN ile Deep Linking kurun veya dizi ID'lerini toplu olarak yeniden eşlemek için FastComments desteğiyle iletişime geçin.

**Öğrenci lansmanda genel bir Blackboard hatası görüyor.** Nedeni, eksik veya bayat bir `email` claim'idir. FastComments için kurum politikasında **User Fields to Send** altında **Role**, **Name** ve **Email Address** seçeneklerinin etkin olduğunu doğrulayın. Kaydedin, ardından yeni bir tarayıcı oturumunda tekrar lansman yapın.