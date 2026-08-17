Once an administrator has registered FastComments as an LTI 1.3 Advantage tool and approved the institution policies, instructors add it to courses through the standard Blackboard placement points. The exact steps differ between Ultra Course View and Original Course View, so both are covered below.

#### Ultra Course View

Ultra Course View, 2026 itibarıyla Blackboard Learn SaaS'te varsayılan görünümüdür.

1. Kursa girin ve **Course Content** (Kurs İçeriği) sayfasına gidin.
2. Yorum dizisinin yer almasını istediğiniz yerde üzerine gelin veya dokunun ve mor **+** (Add content) düğmesine tıklayın.
3. **Content Market**'i seçin. Content Market paneli, kurumunuz için onaylanmış tüm LTI araçlarını ve Building Block yerleştirmelerini listeler.
4. **FastComments** kutusunu bulun ve tıklayın. Blackboard, **+** menüsünü açtığınız konumda bir içerik öğesi oluşturur.
5. Öğe, **Hide from students** (Öğrencilerden gizle) kişisel varsayılanı kapalı olan eğitmenler için varsayılan olarak "Visible to students" (Öğrencilere Görünür) girişi olarak taslakta yer alır. Varsayılanınız **Hidden** (Gizli) ise, öğe gizli olarak oluşturulur ve hazır olduğunuzda öğe satırındaki görünürlük seçicisini değiştirirsiniz.
6. Öğenin adını değiştirmek için, taslaktaki başlığa tıklayın ve yeni bir etiket girin. Öğrencilerin taslaktaki başlığı, FastComments dizisi tanımlayıcısından bağımsızdır, bu yüzden yeniden adlandırma her zaman güvenlidir.

**Content Market** seçeneğini görmüyorsanız, kurumunuz bu yerleştirmeyi gizlemiş demektir. Aynı seçiciyi, **LTI Tools** (LTI Araçları) grubunun altındaki aynı **+** menüsünde **More tools** (Daha fazla araç) aracılığıyla da ulaşabilirsiniz.

#### Original Course View

Original Course View (Orijinal Kurs Görünümü) hâlâ Learn SaaS'te desteklenmekte ve Q4 2024 CU sürüm hattındaki kendi kendine barındırılan Learn 9.1 siteleri için birincil deneyim olarak kalmaktadır.

1. Kursa girin ve bir **Content Area** (örneğin, kurs menüsündeki varsayılan **Information** (Bilgi) veya **Content** (İçerik) alanı) içine girin.
2. Sayfanın sağ üst köşesindeki geçiş düğmesiyle **Edit Mode** (Düzenleme Modu) özelliğini açın.
3. Eylem çubuğunda **Build Content** (İçerik Oluştur) üzerine tıklayın.
4. **Learning Tools** (Öğrenme Araçları) alt menüsünün altında **FastComments**'a tıklayın. Learning Tools alt menüsü, bir yönetici aracı kaydettikten sonra LTI 1.3 araç yerleştirmelerinden doldurulur. Görmüyorsanız, aşağıdaki sorunlar bölümüne bakın.
5. **Create FastComments** (FastComments Oluştur) formunda şunları ayarlayın:
   - **Name** (İsim): öğrencilerin içerik alanında gördüğü etiket.
   - **Description** (Açıklama): gömülü dizinin üstünde gösterilen isteğe bağlı metin.
   - **Permit Users to View this Content** (Kullanıcıların Bu İçeriği Görmesine İzin Ver): Evet/Hayır kullanılabilirlik geçişi.
   - **Track Number of Views** (Görüntüleme Sayısını İzle): Blackboard'un öğe başına görüntüleme istatistiklerini istiyorsanız etkinleştirin. FastComments kendi analizlerini bağımsız olarak yürütür.
   - **Date and Time Restrictions** (Tarih ve Saat Kısıtlamaları): isteğe bağlı **Display After** (Sonra Göster) / **Display Until** (Kadar Göster) pencereleri.
6. Gönderin. Araç, içerik alanında tıklanabilir bir öğe olarak görünür.

#### Embedding Inside an Item or Document

Her iki kurs görünümünde de, eğitmenler FastComments'ı bir Öğenin, Belgenin veya herhangi bir zengin metin alanının gövdesine, Content Editor'ün LTI Advantage düğmesi aracılığıyla satır içi olarak gömer.

**Ultra Course View:**

1. Bir **Document** (Belge) oluşturun veya düzenleyin.
2. Dizinin görünmesini istediğiniz belge gövdesinde **Add content** (İçerik Ekle) üzerine tıklayın.
3. Editör araç çubuğunda, **Insert content** (İçerik Ekle) menüsünü açın ve **Content Market**'e (LTI Advantage / Derin Bağlantı giriş noktası) tıklayın.
4. **FastComments**'ı seçin. FastComments bir derin-bağlantı yükü döndürür ve Blackboard, imleç konumunda belge gövdesine gömülü bir blok ekler.
5. Belgeyi kaydedin. Öğrenciler, diziyi kaydırken satır içi olarak render edilmiş görürler.

**Original Course View:**

1. Zengin metin gövdesi olan herhangi bir öğeyi düzenleyin.
2. Content Editor araç çubuğunda, **Add Content** (İçerik Ekle) artı simgesine tıklayın ve **Content Market**'i seçin (eski Q4 2024 CU'larda **Add Content from External Tool** olarak etiketlenir).
3. **FastComments**'ı seçin. Editör, derin-bağlantılı kaynağa referans veren bir yer tutucu blok ekler.
4. Öğeyi gönderin.

Her derin-bağlantı gömme kendi FastComments dizisini üretir, bu yüzden iki gömülü FastComments bloğu olan bir Öğenin iki bağımsız yorum akışı vardır.

#### Visibility, Release Conditions, and Group Restrictions

FastComments içerik öğeleri, üzerlerine uygulanan erişim kontrol kuralları açısından diğer Blackboard içerik öğeleri gibi davranır.

- Ultra: satırdaki görünürlük seçicisine tıklayın (**Visible to students** (Öğrencilere Görünür), **Hidden from students** (Öğrencilerden Gizli), **Conditional availability** (Koşullu kullanılabilirlik)). Koşullu kullanılabilirlik tarih/saat pencerelerini, not defteri öğelerine karşı performans kurallarını ve kurs gruplarına karşı üye kurallarını destekler.
- Original: öğenin bağlam menüsünü açın ve aracı tarih, üyelik, not veya inceleme durumuna göre kısıtlamak için **Adaptive Release** (Uyarlanabilir Yayın) veya **Adaptive Release: Advanced** (Uyarlanabilir Yayın: Gelişmiş) seçeneğini seçin. Belirli kurs gruplarıyla sınırlamak için öğe üzerinde **Set Group Availability** (Grup Kullanılabilirliğini Ayarla) kullanın.

FastComments, Blackboard'un kapısının ne karar verirse ona saygı gösterir. Blackboard öğeyi bir öğrenciden gizlerse, LTI başlatması o öğrenci için gerçekleşmez ve öğrenci moderatör görünümünde görünmez.

#### Gradebook Behavior

FastComments, LTI Advantage Assignment ve Grade Services üzerinden notları geri bildirmez. FastComments içerik öğeleri için otomatik bir not sütunu oluşturulmaz.

Blackboard kiracınız, notlandırma meta verileri ne olursa olsun her yeni içerik öğesi için otomatik bir not defteri sütunu oluşturacak şekilde yapılandırılmışsa, yine de boş bir sütun görünür. Bunu gizlemek için:

- Ultra: **Gradebook**'u (Not Defteri) açın, sütun başlığına tıklayın, **Edit** (Düzenle) seçeneğini seçin ve **Show to students** (Öğrencilere göster) ile **Include in calculations** (Hesaplamalara dahil et) seçeneklerini kapatın. Notlandırılmamış öğeler için sütun silmeye kurumunuz izin veriyorsa **Delete** (Sil) seçeneğini kullanın.
- Original: **Grade Center**'ı (Not Merkezi) açın, sütunun şerit simgesine tıklayın, **Hide from Users (on/off)** (Kullanıcılardan Gizle (aç/kapat)) seçeneğini seçin ve isteğe bağlı olarak **Column Organization** (Sütun Organizasyonu) altında **Hide from Instructor View** (Eğitmen Görünümünden Gizle) seçeneğini işaretleyin.

#### What Students See

Bir öğrenci FastComments öğesini açtığında veya gömülü bir bloğa kaydırdığında:

1. Blackboard, FastComments'a LTI 1.3 mesajını başlatır. Öğrenci, bir giriş formu görmeden Blackboard kimliği (ad, e-posta, avatar, rol) ile SSO üzerinden oturum açar.
2. Yorum dizisi iframe içinde render edilir. Dizi oluşturma, yanıtlar, bahsetmeler ve tepkiler, FastComments'ta yapılandırılmış yorum widget ayarlarına göre mevcuttur.
3. Yorumları Blackboard hesaplarına atfedilir. Öğrenci daha sonra Blackboard'ta adını veya fotoğrafını değiştirirse, bir sonraki başlatma FastComments profilini günceller.

Blackboard'tan FastComments'a rol eşlemesi:

- **System Administrator** (Sistem Yöneticisi) ve **Course Builder** (Kurs Oluşturucu) FastComments **admin** (yönetici) rolüne eşlenir.
- **Instructor** (Eğitmen) ve **Teaching Assistant** (Öğretim Asistanı) FastComments **moderator** (moderator) rolüne eşlenir.
- **Student** (Öğrenci), **Guest** (Misafir) ve **Observer** (Gözlemci) FastComments **commenter** (yorumcu) rolüne eşlenir.

Moderatorler, dizideki her yorumda satır içi olarak (pin, hide, ban, delete) denetimlerini görür.

#### Lock Down Public Access (Recommended)

Varsayılan olarak, FastComments yorum verileri herkese açık olarak okunabilir. Bir dizinin URL'sini veya API uç noktasını tahmin edebilen herkes, Blackboard dışındaki yorumları da görebilir. Kurs tartışmaları için görüntülemeyi yalnızca kayıtlı öğrencilere sınırlamak isteyeceksiniz.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> (widget özelleştirme sayfasını) açın ve **Require SSO To View Comments** (Yorumları Görmek İçin SSO Gerektir) etkinleştirilmiş bir kural oluşturun, ardından güvenlik seviyesini **Secure SSO** (Güvenli SSO) olarak ayarlayın, böylece diziler yalnızca imzalı LTI başlatmasıyla yüklenebilir.

Tam yürütme için, kuralı tek bir alan adına veya sayfaya nasıl sınırlayacağınızı içeren [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) (Yorum Dizilerini Tek Oturum Açma ile Koruma) bölümüne bakın.

#### Thread Scoping

FastComments, her diziyi **(Blackboard host, course ID, resource link ID)** (Blackboard sunucusu, kurs kimliği, kaynak bağlantı kimliği) ile sınırlar. Aynı kurstaki iki FastComments öğesi iki dizi üretir. Aynı öğe iki kurs kabuğuna (örneğin, kurs kopyalama yoluyla) kopyalandığında iki dizi üretir, çünkü Blackboard kopyalama sırasında yeni bir kaynak bağlantı kimliği verir. Kurs kopyaları arasında bir dizi paylaşmak için, kopyalamayı başlatmadan önce FastComments'ta yapılandırılmış açık bir dizi URN ile Derin Bağlantı (Deep Linking) kullanın.

#### Blackboard-Specific Gotchas

- **FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** (FastComments karosu, Build Content menüsünden (Original) veya Content Market'ten (Ultra) eksik.) Yönetici aracı onaylamış ancak ilgili yerleştirmeyi engelleyen bir kurum politikası bırakmıştır. **Administrator Panel** > **Integrations** > **LTI Tool Providers** (Yönetici Paneli > Entegrasyonlar > LTI Araç Sağlayıcıları) yoluna gidin, FastComments girişini düzenleyin ve hem **Course Content Tool** (Original) hem de **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) yerleştirmelerinin etkin olduğundan emin olun. Kaydedin ve kurs sayfasını yenileyin.
- **"Tool not configured for this context" or "Tool is not deployed" error on launch.** (Başlatma sırasında "Araç bu bağlam için yapılandırılmamış" veya "Araç dağıtılmamış" hatası.) Dinamik kayıt sırasında kaydedilen dağıtım kapsamı, kursun ait olduğu kurum bağlamıyla eşleşmiyor. Blackboard'un araç sağlayıcı girişinde, **Deployment ID** (Dağıtım Kimliği)** FastComments'ın bu kiracı için LTI 1.3 Yapılandırma sayfasında gösterdiğiyle aynı olduğundan emin olun. Farklıysa, yerleştirmeyi silin ve yeni bir kayıt URL'sinden (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alın</a>) dinamik kaydı yeniden çalıştırın.
- **Iframe height looks fixed or content gets cut off.** (Iframe yüksekliği sabit görünüyor veya içerik kesiliyor.) Bazı Blackboard kiracıları, varsayılan LTI iframe-resize postMessage'ı engelleyen katı bir İçerik Güvenlik Politikası (CSP) ile gelir. FastComments, uyumluluğu en üst düzeye çıkarmak için Canvas tarzı `lti.frameResize` mesajı ve IMS spec-form `org.imsglobal.lti.frameResize` mesajını her ikisini de gönderir, ancak kiracı düzeyinde bir CSP geçersiz kılma ebeveyn dinleyiciyi engeller. Yöneticinizden `*.fastcomments.com`'un LTI araç izin listesinde olduğundan ve özel bir CSP başlığının postMessage olaylarını kesmediğinden emin olmasını isteyin. Yeniden boyutlandırma, ek yapılandırma olmadan çalışır.
- **Course copy duplicates threads.** (Kurs kopyası dizileri çoğaltır.) Blackboard kurs kopyası, LTI yerleştirmeleri için yeni kaynak bağlantı kimlikleri verir, bu yüzden kopyalanan kurslar boş dizilerle başlar. Bu beklenen bir durumdur. Kopyalanan kursun orijinal diziyi devralmasını istiyorsanız, kopyalamadan önce açık bir dizi URN ile Derin Bağlantı (Deep Linking) kurun veya toplu olarak dizi kimliklerini yeniden eşlemek için FastComments desteğiyle iletişime geçin.
- **Student sees a generic Blackboard error on launch.** (Öğrenci başlatma sırasında genel bir Blackboard hatası görür.) Nedeni eksik veya eski bir `email` talebidir. FastComments için kurum politikasının **User Fields to Send** (Gönderilecek Kullanıcı Alanları) altında **Role**, **Name** ve **Email Address** (Rol, İsim ve E-posta Adresi) etkinleştirildiğini doğrulayın. Kaydedin, ardından yeni bir tarayıcı oturumunda tekrar başlatın.

---