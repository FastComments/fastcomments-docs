Bir yönetici FastComments'ı bir LTI 1.3 Advantage aracı olarak kaydedip kurum politikalarını onayladıktan sonra, öğretim üyeleri standart Blackboard yerleştirme noktaları aracılığıyla bunu derslere ekler. Tam adımlar Ultra Course View ile Original Course View arasında farklılık gösterdiğinden her ikisi de aşağıda ele alınmıştır.

#### Ultra Kurs Görünümü

Ultra Kurs Görünümü, 2026 itibarıyla Blackboard Learn SaaS'ta varsayılandır.

1. Dersi açın ve **Course Content** sayfasına gidin.
2. Anahattaki yorum dizgisinin yerleşmesini istediğiniz yere fareyle gelin veya dokunun ve mor **+** (İçerik ekle) düğmesine tıklayın.
3. **Content Market** öğesini seçin. Content Market paneli kurumunuz için onaylanmış her LTI aracını ve Building Block yerleşimini listeler.
4. **FastComments** kutucuğunu bulun ve tıklayın. Blackboard, **+** menüsünü açtığınız konumda bir içerik öğesi oluşturur.
5. Öğenin anahatta varsayılan olarak "Visible to students" girdisi olarak yerleşir; bu, kişisel varsayılanı olarak **Hide from students** kapalı olan eğitmenler içindir. Varsayılanınız **Hidden** ise öğe gizli oluşturulur ve hazır olduğunuzda öğe satırındaki görünürlük seçicisini açarsınız.
6. Öğeyi yeniden adlandırmak için anahattaki başlığa tıklayın ve yeni bir etiket yazın. Öğrencilerin anahatta gördüğü başlık, FastComments dizgi tanımlayıcısından bağımsızdır; bu nedenle yeniden adlandırma her zaman güvenlidir.

**Content Market** seçeneğini görmüyorsanız, kurumunuz yerleşimi gizlemiş demektir. Aynı seçim aracına aynı **+** menüsündeki **More tools** altında **LTI Tools** grubundan da ulaşabilirsiniz.

#### Orijinal Kurs Görünümü

Orijinal Kurs Görünümü, Learn SaaS'ta hâlâ desteklenmektedir ve Q4 2024 CU sürüm hattındaki kendi sunucusunda barındırılan Learn 9.1 siteleri için birincil deneyim olmaya devam etmektedir.

1. Dersi açın ve bir **Content Area** içine girin (örneğin, ders menüsündeki varsayılan **Information** veya **Content** alanı).
2. Sayfanın sağ üst köşesindeki anahtarla **Edit Mode**'u açın.
3. Eylem çubuğunda **Build Content** öğesine tıklayın.
4. **Learning Tools** alt menüsünde **FastComments**'a tıklayın. Learning Tools alt menüsü, bir yönetici aracı kaydettikten sonra LTI 1.3 araç yerleşimlerinden doldurulur. Görmüyorsanız, aşağıdaki dikkat edilmesi gerekenlere bakın.
5. **Create FastComments** formunda şunları ayarlayın:
   - **Name**: içerik alanında öğrencilerin göreceği etiket.
   - **Description**: gömülü dizginin üstünde gösterilen isteğe bağlı metin.
   - **Permit Users to View this Content**: Evet/Hayır erişilebilirlik anahtarı.
   - **Track Number of Views**: Blackboard'un öğe başına görüntüleme istatistiklerini istiyorsanız etkinleştirin. FastComments kendi analizlerini bağımsız olarak çalıştırır.
   - **Date and Time Restrictions**: isteğe bağlı **Display After** / **Display Until** pencereleri.
6. Gönderin. Araç içerik alanında tıklanabilir bir öğe olarak görünür.

#### Bir Öğenin veya Belgenin İçine Gömme

Her iki kurs görünümünde de öğretim üyeleri Content Editor'deki LTI Advantage düğmesi aracılığıyla FastComments'ı bir Öğenin, Belgenin veya herhangi bir zengin metin alanının gövdesine satır içi olarak gömerler.

Ultra Kurs Görünümü:

1. Bir **Document** oluşturun veya düzenleyin.
2. Dizginin görünmesini istediğiniz yerde belgenin gövdesinde **Add content**'e tıklayın.
3. Editör araç çubuğunda **Insert content** menüsünü açın ve **Content Market**'e tıklayın (LTI Advantage / Deep Linking giriş noktası).
4. **FastComments**'ı seçin. FastComments bir derin-bağlantı yükü (deep-link payload) döndürür ve Blackboard, belge gövdesinde imleç konumuna gömülü bir blok ekler.
5. Belgeyi kaydedin. Öğrenciler belgeyi kaydırırken dizgiyi satır içinde render edilmiş olarak görürler.

Orijinal Kurs Görünümü:

1. Zengin metin gövdeli herhangi bir öğeyi düzenleyin.
2. Content Editor araç çubuğunda **Add Content** artı simgesine tıklayın ve **Content Market**'i seçin (eski Q4 2024 CU'larda **Add Content from External Tool** olarak etiketlenir).
3. **FastComments**'ı seçin. Editör, derin-bağlanan kaynağa referans veren bir yer tutucu blok ekler.
4. Öğeyi gönderin.

Her derin-bağlantı gömme kendi FastComments dizgisini üretir; bu nedenle içinde iki gömülü FastComments bloğu olan bir Öğenin iki bağımsız yorum akışı olur.

#### Görünürlük, Yayınlama Koşulları ve Grup Kısıtlamaları

FastComments içerik öğeleri, üzerinde katmanlanmış erişim kontrol kuralları açısından diğer Blackboard içerik öğeleri gibi davranır.

- Ultra: satırdaki görünürlük seçicisine tıklayın (**Visible to students**, **Hidden from students**, **Conditional availability**). Koşullu erişilebilirlik tarih/saat pencerelerini, not defteri öğelerine karşı performans kurallarını ve ders gruplarına karşı üye kurallarını destekler.
- Original: öğenin bağlam menüsünü açın ve aracı tarih, üyelik, not veya inceleme durumuna göre sınırlandırmak için **Adaptive Release** veya **Adaptive Release: Advanced**'i seçin. Öğeyi belirli ders gruplarıyla kısıtlamak için öğede **Set Group Availability**'i kullanın.

FastComments, Blackboard'un hangi kapıyı etkinleştirdiğini esas alır. Blackboard öğeyi bir öğrenciden gizlerse, o öğrenci için LTI başlatması hiç gerçekleşmez ve moderator görünümünde görünmezler.

#### Not Defteri (Gradebook) Davranışı

FastComments, LTI Advantage Assignment and Grade Services üzerinden not bildirmez. FastComments içerik öğeleri için otomatik olarak bir not sütunu oluşturulmaz.

Eğer Blackboard kiracınız her yeni içerik öğesi için derecelendirme meta verisinden bağımsız olarak bir not defteri sütunu otomatik oluşturacak şekilde yapılandırıldıysa, yine boş bir sütun görünür. Gizlemek için:

- Ultra: **Gradebook**'u açın, sütun başlığına tıklayın, **Edit**'i seçin ve **Show to students** ile **Include in calculations**'ı kapatın. Ya da kurumunuz notlandırılmamış öğeler için sütun silmeyi izin veriyorsa **Delete** kullanın.
- Original: **Grade Center**'ı açın, sütunun üçgenine tıklayın, **Hide from Users (on/off)**'u seçin ve gerekirse **Column Organization** altında **Hide from Instructor View**'u seçin.

#### Öğrencilerin Gördüğü Şey

Bir öğrenci FastComments öğesini açtığında veya gömülü bloğa kaydırdığında:

1. Blackboard, FastComments'a LTI 1.3 mesajını başlatır. Öğrenci, giriş formu görmeden Blackboard kimliği (isim, e-posta, avatar, rol) kullanılarak SSO ile oturum açmış olur.
2. Yorum dizgisi iframe içinde render edilir. Dizileme, yanıtlar, bahsetmeler ve tepkiler, FastComments içinde yapılandırılmış yorum widget ayarlarına bağlı olarak kullanılabilir.
3. Yorumları Blackboard hesaplarına atfedilir. Öğrenci daha sonra Blackboard'da adını veya fotoğrafını düzenlerse, bir sonraki başlatma FastComments profilini günceller.

Blackboard'dan FastComments'a rol eşlemesi:

- **System Administrator** ve **Course Builder**, FastComments **admin** rolüne eşlenir.
- **Instructor** ve **Teaching Assistant**, FastComments **moderator** rolüne eşlenir.
- **Student**, **Guest** ve **Observer**, FastComments **commenter** rolüne eşlenir.

Moderatörler, dizgideki her yorumun yanında yer alan moderasyon kontrollerini (pinleme, gizleme, ban, silme) görürler.

#### Dizgi Kapsamlandırma

FastComments her dizgiyi **(Blackboard host, course ID, resource link ID)** ile kapsar. Aynı derste iki FastComments öğesi iki dizgi üretir. Aynı öğe iki ders kabuğuna kopyalandığında (örneğin, ders kopyasıyla) iki dizgi oluşur, çünkü Blackboard kopyalama sırasında yeni bir resource link ID verir. Ders kopyaları arasında paylaşılan bir dizgi tutmak için, kopyalamadan önce FastComments içinde açık bir dizgi URN'si ile Derin Bağlama (Deep Linking) kullanın.

#### Blackboard'a Özgü Dikkat Edilmesi Gerekenler

**FastComments kutucuğu Build Content menüsünde (Orijinal) veya Content Market'te (Ultra) yok.** Yönetici aracı onayladı ama ilgili yerleşimi engelleyen bir kurum politikası bırakmış. **Administrator Panel** > **Integrations** > **LTI Tool Providers** yoluna gidin, FastComments girişini düzenleyin ve hem **Course Content Tool** (Original) hem de **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) yerleşimlerinin etkinleştirildiğini doğrulayın. Kaydedin ve ders sayfasını yenileyin.

**Başlatmada "Tool not configured for this context" veya "Tool is not deployed" hatası.** Dinamik kayıt sırasında kaydedilen dağıtım kapsamı, dersin ait olduğu kurum bağlamıyla eşleşmiyor. Blackboard'daki araç sağlayıcı girdisinde **Deployment ID**'nin bu kiracı için FastComments'ın LTI 1.3 Yapılandırma sayfasında gösterdiğiyle eşleştiğini doğrulayın. Eğer farklıysa, yerleşimi silin ve taze bir kayıt URL'sinden dinamik kaydı yeniden çalıştırın.

**Iframe yüksekliği sabit görünüyor veya içerik kesiliyor.** Bazı Blackboard kiracıları, varsayılan LTI iframe yeniden boyutlandırma postMessage'ını engelleyen katı bir Content Security Policy ile gönderilir. FastComments uyumluluğu maksimize etmek için hem Canvas tarzı `lti.frameResize` mesajını hem de IMS spesifikasyon formu `org.imsglobal.lti.frameResize` mesajını gönderir, ancak kiracı düzeyindeki bir CSP üstbilgisi üst dinleyiciyi engelliyor olabilir. Yöneticinizden `*.fastcomments.com`'un LTI araç izinli listesinde olduğunu ve hiçbir özel CSP üstbilgisinin postMessage olaylarını kesmediğini doğrulamasını isteyin. Böylece yeniden boyutlandırma ek yapılandırma olmadan çalışır.

**Ders kopyası dizgileri çoğaltıyor.** Blackboard ders kopyası LTI yerleşimleri için yeni resource link ID'leri verir, bu nedenle kopyalanan dersler boş dizgilerle başlar. Bu beklenen bir davranıştır. Kopyalanan dersin orijinal dizgiyi devralmasını istiyorsanız, kopyalamadan önce açık bir dizgi URN'si ile Derin Bağlama kurun veya toplu olarak dizgi kimliklerini yeniden eşleme için FastComments destek ile iletişime geçin.

**Öğrenci başlatmada genel bir Blackboard hatası görüyor.** Bunun nedeni eksik veya eski bir `email` iddiasıdır (claim). FastComments için kurum politikasında **User Fields to Send** altında **Role**, **Name** ve **Email Address**'in etkinleştirildiğini doğrulayın. Kaydedin, ardından taze bir tarayıcı oturumunda tekrar başlatın.