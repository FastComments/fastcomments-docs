---
Bu sayfa, bir yönetici aracılığı kaydı yapıp bir dağıtım oluşturduktan sonra FastComments'ı bir Brightspace dersine eklemeyi kapsar. Araç henüz kaydedilmediyse önce D2L kayıt kılavuzuna bakın.

Brightspace iki içerik oluşturma deneyimi sunar: **Klasik İçerik** ve **Yeni İçerik Deneyimi** (aynı zamanda **Lessons** olarak da adlandırılır). Her ikisi de FastComments'ı sunar, ancak menü yolları farklıdır. Aşağıdaki her bölüm, ayrıldıkları yerlerde her ikisini de kapsar.

#### FastComments Araçını Bulma

FastComments aracı, ders içerik düzenleyicisi içinde iki yerde görünür:

1. Bir modül/birimdeki **Add Existing** düğmesinden ulaşılan etkinlik seçicisinde (eski Brightspace sürümlerinde **Add Existing Activities** olarak etiketlenmiştir). Güncel Brightspace sürümlerinde FastComments seçicide doğrudan görünür; eski sürümler bunu **External Learning Tools** alt menüsü altında içerebilir. Her iki yol da FastComments'ı bağımsız bir konu olarak ekler.
2. HTML düzenleyicisi içindeki **Insert Stuff** iletişim kutusunda, **LTI Advantage** altında. Bu, LTI derin bağlantı akışı aracılığıyla FastComments'ı bir HTML konusu içine satır içi olarak yerleştirir.

FastComments her iki seçicide de görünmüyorsa, dağıtım dersin bulunduğu organizasyon birimi için etkinleştirilmemiş demektir. Brightspace yöneticinize şunu yapmasını söyleyin: **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments aracı > **View Deployments**, dağıtımı açın ve dersin org birimini (veya üst org birimini) **Org Units** altına ekleyin.

#### FastComments'ı Bir Modülde Konu Olarak Ekleme

Klasik İçerik:

1. Dersi açın ve navigasyon çubuğunda **Content**'e tıklayın.
2. Tartışmanın yer alacağı modülü seçin (veya **Add a module** ile oluşturun).
3. **Add Existing**'e tıklayın (eski Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Seçicide **FastComments**'a tıklayın. Brightspace modülde bir konu oluşturur ve sizi içerik görünümüne geri götürür.
5. Yeni konuya tıklayın. Satır içi başlık düzenleyicisini kullanarak adını `FastComments Discussion` gibi açıklayıcı bir şeyle değiştirin.

Yeni İçerik Deneyimi (Lessons):

1. Dersi açın ve **Content**'e tıklayın.
2. Tartışmanın yer alacağı birimi ve dersi açın.
3. **Add** > **Existing Activity**'ye tıklayın ve **FastComments**'ı seçin (eski Brightspace: **External Learning Tools** altında iç içe).
4. Etkinlik derse eklenir.
5. Yeniden adlandırmak için etkinlik başlığına tıklayın.

Herhangi bir kullanıcı (öğretim üyesi veya öğrenci) konuyu ilk kez açtığında, FastComments o kaynak bağlantısı için konuyu başlatır. Konu, kaynak bağlantısı kimliğine bağlıdır; bu nedenle konunun yeniden adlandırılması veya taşınması hangi konunun yükleneceğini değiştirmez.

#### FastComments'ı Bir HTML Konusu İçine Satır İçi Olarak Yerleştirme

Yorumların ayrı bir konu olarak değil, aynı konu sayfasında bir okumanın, videonun veya başka içeriğin altında görünmesini istediğinizde bu akışı kullanın.

1. Modülde/derste bir HTML konusu açın veya oluşturun.
2. Brightspace HTML düzenleyicisini açmak için **Edit HTML**'ye tıklayın.
3. Yorum akışının görünmesini istediğiniz yere imleci yerleştirin.
4. **Insert Stuff** düğmesine tıklayın (düzenleyici araç çubuğundaki yapboz parçası simgesi).
5. Insert Stuff iletişim kutusunda **LTI Advantage**'a gidin ve **FastComments**'a tıklayın.
6. FastComments bir derin bağlantı seçici açar. Yerleşimi onaylayın (varsayılan seçenekler içerik tartışmaları için uygundur); **Insert** veya **Continue**'a tıklayın.
7. Brightspace, LTI başlatmasını temsil eden bir yer tutucuyla HTML düzenleyicisine geri döner. Konuda **Save and Close**'a tıklayın.

Konu yüklendiğinde, Brightspace yer tutucuyu bir iframe ile değiştirir ve LTI aracılığıyla FastComments'ı otomatik olarak başlatır. Öğrenciler tartışma konusunu satır içi olarak görür.

Tek bir HTML konusu, birden fazla derin bağlantılı FastComments yerleştirmesini barındırabilir. Her yerleştirme kendi konusu olur çünkü her derin bağlantı farklı bir kaynak bağlantısı kimliği üretir.

#### Modül Konusu vs Satır İçi Hızlı Bağlantı

Aşağıdaki durumlarda **modül konusu** yaklaşımını seçin:

- Tartışma, modüldeki adım için birincil etkinlikse.
- Konunun Brightspace'in içerik tablosunda, tamamlanma takibinde ve Class Progress'te görünmesini istiyorsunuz.

Aşağıdaki durumlarda **satır içi yerleştirme** yaklaşımını seçin:

- Yorumların aynı sayfada diğer içeriğin altında bulunması gerekiyorsa.
- İçerik tablosunda ayrı, tamamlanma takibi yapılabilen bir öğe istemiyorsanız.

#### Görünürlük, Taslak ve Yayınlama Koşulları

Yeni bir FastComments konusu varsayılan olarak öğrencilere görünür. Kurulum sırasında gizlemek için:

1. İçerik düzenleyicisinde konu başlığına tıklayın (Klasik) veya etkinlikteki üç nokta menüsüne tıklayın (Yeni İçerik Deneyimi).
2. Durumu **Draft** olarak ayarlayın (Klasik) veya **Visibility**'yi kapatın (Yeni İçerik Deneyimi).

Taslak konular öğrencilere görünmez. Eğitmenler ve yardımcı öğretim görevlileri yine de "Draft" rozetini görürler.

Konuyu belirli bir grup veya bölüme kısıtlamak için:

1. Konuyu açın.
2. Konu başlığı menüsünde > **Edit Properties In-place** (Klasik) veya **Edit** > **Restrictions** (Yeni İçerik Deneyimi) seçeneğine tıklayın.
3. **Release Conditions** altında **Create**'e tıklayın.
4. **Group enrollment** veya **Section enrollment**'ı seçin, grup/bölümü seçin ve kaydedin.

Yayınlama koşulları, FastComments'ın kendi rol eşlemesiyle üst üste biner. Konuyu göremeyen öğrenciler LTI başlatması alamazlar.

#### Öğrenciler İlk Başlatmada Ne Görür

Bir öğrenci konuya tıkladığında (veya yerleştirilmiş bir HTML konusunu yüklediğinde):

1. Brightspace arka planda LTI 1.3 başlatmasını gerçekleştirir.
2. FastComments, öğrencinin adını, e-posta adresini, avatar URL'sini ve LMS rolünü alır ve onları otomatik olarak oturum açtırır. FastComments oturum açma istemi yoktur.
3. O kaynak bağlantısı için yorum konusu Brightspace iframe'i içinde render edilir.

Başlatmadaki rol eşlemesi:

- Brightspace `Administrator` o konu için FastComments içinde bir **admin** olur (tam moderasyon, silme, engelleme ve yapılandırma erişimi).
- Brightspace `Instructor` bir FastComments **moderator** olur (sabitleme, gizleme, silme, engelleme).
- Diğer tüm roller (`Learner`, `TeachingAssistant`, vb.) standart yorumcu olurlar.

Yorumlar öğrencinin Brightspace hesabına atfedilir. Öğrenci Brightspace'te adını veya avatarını düzenlerse, bir sonraki LTI başlatması değişikliği senkronize eder.

#### Iframe Yüksekliği ve Yeniden Boyutlandırma

FastComments, her konu render'ında ve içerik değişikliklerinde (yeni yorum, yanıtları genişletme) `org.imsglobal.lti.frameResize` postMessage'ını gönderir. Brightspace bu mesajı dinler ve iframe yüksekliğini ayarlar, böylece konu kesilmez ve iç kaydırma çubuğu görünmez.

Iframe sabit, kısa bir yükseklikte kalıyorsa:

- Dersin HTTPS üzerinden yüklendiğini doğrulayın. Brightspace'in postMessage dinleyicisi karışık içeriği reddeder.
- Hiçbir tarayıcı uzantısının postMessage kanalını engellemediğini doğrulayın.
- Bir HTML konusundaki satır içi yerleştirmeler için, çevreleyen HTML iframe'i sabit yüksekliğe sahip bir kapsayıcıya sarmamalıdır. Üst öğeden herhangi bir inline `style="height: ..."` kaldırın.

#### Brightspace'e Özgü Tuzaklar

**Araç Add Existing seçicide görünmüyor.** Dağıtım bu dersin org birimi için etkinleştirilmemiş demektir. Bir yönetici dağıtımın **Org Units** listesine org birimini (veya bir üstünü) eklemelidir. Sadece araç kaydı yeterli değildir; dağıtım hangi derslerin aracı gördüğünü belirler.

**`deployment_id` mismatch on launch.** FastComments, bir kayıt için gördüğü ilk `deployment_id`'yi TOFU şeklinde sabitler. Bir yönetici orijinal dağıtımı silip yenisini oluşturursa, yeni dağıtımdan yapılan başlatmalar dağıtım uyumsuzluğu hatasıyla reddedilir. Çözüm, FastComments'ı yeniden kaydetmektir (yeni bir kayıt URL'si oluşturup Dinamik Kayıt'ı tekrar çalıştırın); eski yapılandırma kaydı değiştirilir.

**Araç başlıyor ama "Invalid LTI launch" gösteriyor.** Ders, dağıtımın kapsadığı tenant/org yapısının dışında olabilir veya dağıtım kayıt sonrası devre dışı bırakılmış olabilir. Tekrar kontrol edin: **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** geçişi ve dağıtımın org birimi listesi.

**İsimler ve roller FastComments içinde eksik.** Brightspace, LTI başlatmalarını Names and Role Provisioning Services (NRPS) iddialarıyla gönderir. Bir ders eski bir LTI 1.1 bağlantısından yükseltildiyse, başlatma `name` ve `email` iddialarından yoksun olabilir. FastComments konusunu **Add Existing** ile yeniden ekleyin (eski bağlantıyı taşımayın) böylece başlatma LTI 1.3 kullanır.

**Embed otomatik SSO yerine bir giriş ekranı gösteriyor.** HTML konusu, **Insert Stuff** > **LTI Advantage** yerine doğrudan FastComments'a işaret eden düz bir `<iframe>` olarak eklenmiş olabilir. Düz iframeler LTI başlatmasını atlar ve kullanıcıları genel FastComments sayfasına götürür. Iframe'i silin ve Insert Stuff akışıyla yeniden ekleyin.

---