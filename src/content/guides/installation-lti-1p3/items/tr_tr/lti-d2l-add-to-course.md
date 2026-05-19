Bu sayfa, bir yönetici aracı kaydettikten ve bir dağıtım oluşturduktan sonra FastComments'ın Brightspace dersine nasıl ekleneceğini açıklar. Araç henüz kaydedilmediyse önce D2L kayıt kılavuzuna bakın.

<div class="screenshot white-bg">
    <div class="title">Brightspace içinde bir ünite konusu olarak gömülü FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace iki içerik oluşturma deneyimi sunar: **Classic Content** ve **New Content Experience** (diğer adıyla **Lessons**). Her ikisi de FastComments'ı sağlar, ancak menü yolları farklıdır. Aşağıdaki her bölüm, ayrıldıkları yerlerde her ikisini de kapsar.

#### FastComments Araçını Bulun

FastComments aracı, ders içerik düzenleyicisinde iki yerde görünür:

1. Bir modül/ünitenin **Add Existing** düğmesinden (eski Brightspace sürümlerinde **Add Existing Activities** olarak etiketlenir) erişilen etkinlik seçicisinde. FastComments, mevcut Brightspace derlemelerinde seçicide doğrudan görünür; eski sürümlerde **External Learning Tools** alt menüsü altında yer alır. Her iki yol da FastComments'ı bağımsız bir konu olarak ekler.
2. HTML düzenleyicisi içindeki **Insert Stuff** iletişim kutusunda, **LTI Advantage** altında. Bu, LTI derin bağlantı akışıyla FastComments'ı bir HTML konusu içinde satır içi olarak gömer.

FastComments her iki seçim kutusunda da görünmüyorsa, dağıtım kursu barındıran organizasyon birimi için etkinleştirilmemiş demektir. Brightspace yöneticinizden **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments aracı > **View Deployments** öğesini açmasını, dağıtımı açmasını ve kursun organizasyon birimini (veya üst bir org birimini) **Org Units** altında eklemesini isteyin.

#### FastComments'ı Bir Modüle Konu Olarak Ekleme

Classic Content:

1. Dersi açın ve gezinti çubuğunda **Content**'e tıklayın.
2. Tartışmanın yer alacağı modülü seçin (veya **Add a module** ile oluşturun).
3. **Add Existing**'e tıklayın (eski Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Seçicide **FastComments**'a tıklayın. Brightspace modülde bir konu oluşturur ve sizi içerik görünümüne geri götürür.
5. Yeni konuya tıklayın. Satır içi başlık düzenleyicisini kullanarak adını `FastComments Discussion` gibi açıklayıcı bir şeyle değiştirin.

New Content Experience (Lessons):

1. Dersi açın ve **Content**'e tıklayın.
2. Tartışmanın yer alacağı ünite ve dersi açın.
3. **Add** > **Existing Activity**'ye tıklayın ve **FastComments**'ı seçin (eski Brightspace: **External Learning Tools** altında iç içe).
4. Etkinlik derse eklenir.
5. Etkinlik başlığına tıklayarak yeniden adlandırın.

Herhangi bir kullanıcı (eğitmen veya öğrenci) konuyu ilk açtığında, FastComments o kaynak bağlantısı için konuyu başlatır. Konu, kaynak bağlantısı kimliğiyle bağlanır, bu nedenle konunun adını değiştirmek veya taşımak hangi konunun yüklendiğini değiştirmez.

#### FastComments'ı Bir HTML Konusunda Satır İçi Gömme

Yorumların ayrı bir konu olarak değil, aynı konu sayfasındaki bir okuma, video veya diğer içeriğin altında görünmesini istediğinizde bu akışı kullanın.

1. Modülde/ders içinde bir HTML konusu açın veya oluşturun.
2. Brightspace HTML düzenleyicisini açmak için **Edit HTML**'e tıklayın.
3. Yorum konusunun görünmesini istediğiniz yere imleci yerleştirin.
4. **Insert Stuff** düğmesine tıklayın (düzenleyici araç çubuğundaki yapboz parçası simgesi).
5. Insert Stuff iletişim kutusunda **LTI Advantage**'a kadar kaydırın ve **FastComments**'a tıklayın.
6. FastComments bir derin bağlantı seçici açar. Yerleşimi onaylayın (varsayılan seçenekler içerik tartışmaları için uygundur); **Insert** veya **Continue**'a tıklayın.
7. Brightspace, HTML düzenleyicisine bir LTI başlatmayı temsil eden bir yer tutucu blok ile geri döner. Konuda **Save and Close**'a tıklayın.

Konu yüklendiğinde, Brightspace yer tutucuyu bir iframe ile değiştirir ve iframe LTI aracılığıyla FastComments'ı otomatik başlatır. Öğrenciler tartışma konusunu satır içi olarak görür.

Tek bir HTML konu, birden fazla derin bağlantılı FastComments gömmesi barındırabilir. Her gömme kendi konusunu alır çünkü her derin bağlantı ayrı bir kaynak bağlantısı kimliği üretir.

#### Modül Konusu vs Satır İçi Hızlı Bağlantı

Aşağıdaki durumlarda **modül konusu** yaklaşımını seçin:

- Tartışma, modüldeki adım için birincil etkinlikse.
- Konunun Brightspace içerik tablosunda, tamamlama takibinde ve Class Progress içinde görünmesini istiyorsunuz.

Aşağıdaki durumlarda **satır içi gömme** yaklaşımını seçin:

- Yorumların aynı sayfadaki diğer içeriklerin altında yer almasını istiyorsunuz.
- İçindekiler tablosunda ayrı, tamamlanma izlemeli bir öğe olmasını istemiyorsunuz.

#### Görünürlük, Taslak ve Yayınlama Koşulları

Yeni bir FastComments konusu varsayılan olarak öğrencilere görünür. Kurulum yaparken gizlemek için:

1. İçerik düzenleyicisinde konu başlığına tıklayın (Classic) veya etkinlikteki üç nokta menüsüne tıklayın (New Content Experience).
2. Durumu **Draft** olarak ayarlayın (Classic) veya **Visibility**'yi kapatın (New Content Experience).

Taslak konular öğrenciler için görünmez. Eğitmenler ve yardımcı öğretim elemanları yine de bunları "Draft" rozeti ile görür.

Konuyu belirli bir gruba veya bölüme sınırlamak için:

1. Konuyu açın.
2. Konu başlığı menüsüne tıklayın > **Edit Properties In-place** (Classic) veya **Edit** > **Restrictions** (New Content Experience).
3. **Release Conditions** altında **Create**'a tıklayın.
4. **Group enrollment** veya **Section enrollment** seçin, grup/bölümü seçin ve kaydedin.

Yayın koşulları FastComments'ın kendi rol eşlemesiyle birlikte uygulanır. Konuyu göremeyen öğrenciler LTI başlatması almaz.

#### Öğrencilerin İlk Başlatmada Gördükleri

Bir öğrenci konuya tıkladığında (veya gömme içeren bir HTML konusunu yüklediğinde):

1. Brightspace arka planda LTI 1.3 başlatmasını gerçekleştirir.
2. FastComments, öğrencinin adını, e-posta adresini, avatar URL'sini ve LMS rolünü alır ve onları otomatik olarak oturum açtırır. FastComments için bir oturum açma istemi yoktur.
3. O kaynak bağlantısı için yorum konusu Brightspace iframe'i içinde görüntülenir.

Başlatmadaki rol eşlemesi:

- Brightspace `Administrator` o konu için FastComments içinde bir **admin** olur (tam moderasyon, silme, engelleme ve yapılandırma erişimi).
- Brightspace `Instructor` FastComments içinde bir **moderator** olur (sabitleme, gizleme, silme, engelleme).
- Diğer tüm roller (`Learner`, `TeachingAssistant`, vb.) normal yorumcu olur.

Yorumlar öğrencinin Brightspace hesabına atfedilir. Öğrenci Brightspace'te adını veya avatarını düzenlerse, sonraki LTI başlatması değişikliği senkronize eder.

#### Iframe Yüksekliği ve Yeniden Boyutlandırma

FastComments her konu render'ında ve içerik değişikliklerinde (yeni yorum, yanıtların açılması) `org.imsglobal.lti.frameResize` postMessage'ını gönderir. Brightspace bu mesajı dinler ve iframe yüksekliğini ayarlar, böylece konu kesilmez ve iç kaydırma çubuğu görünmez.

Iframe sabit ve kısa kalıyorsa:

- Kursun HTTPS üzerinden yüklendiğini doğrulayın. Brightspace'in postMessage dinleyicisi karışık içerikli frame'leri reddeder.
- Hiçbir tarayıcı uzantısının postMessage kanalını engellemediğini doğrulayın.
- HTML konuda satır içi gömmeler için, çevreleyen HTML iframe'i sabit yüksekliğe sahip bir konteynerle sarmamalıdır. Üst öğeden herhangi bir inline `style="height: ..."` kaldırın.

#### Brightspace'e Özgü Sorunlar

**Araç Add Existing seçicisinde görünmüyor.** Dağıtım bu kursun organizasyon birimi için etkinleştirilmemiş. Bir yönetici dağıtımın **Org Units** listesine organizasyon birimini (veya üst bir org birimini) eklemelidir. Araç kaydı tek başına yeterli değildir; dağıtım hangi kursların aracı gördüğünü belirler.

**Başlatmada `deployment_id` uyumsuzluğu.** FastComments, bir kayıt için gördüğü ilk `deployment_id`'yi TOFU (Trust On First Use) ile sabitler. Bir yönetici orijinal dağıtımı silip yeni bir tane oluşturursa, yeni dağıtımdan yapılan başlatmalar dağıtım uyumsuzluğu hatası ile reddedilir. Çözüm, FastComments'ı yeniden kaydetmektir (yeni bir kayıt URL'si oluşturun (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan edinin</a>) ve Dinamik Kaydı tekrar çalıştırın); eski yapılandırma kaydı değiştirilir.

**Araç başlıyor ama "Invalid LTI launch" gösteriyor.** Kurs, dağıtımın kapsadığı tenant/organizasyon yapısının dışında olabilir veya dağıtım kayıttan sonra devre dışı bırakılmış olabilir. **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** geçişini ve dağıtımın org birimi listesini yeniden kontrol edin.

**FastComments içinde isimler ve roller eksik.** Brightspace LTI başlatmalarını Names and Role Provisioning Services (NRPS) iddiaları ile gönderir. Bir kurs eski bir LTI 1.1 bağlantısından yükseltildiyse, başlatma `name` ve `email` iddialarından yoksun olabilir. FastComments konusunu **Add Existing** ile yeniden ekleyin (eski bağlantıyı taşımayın) böylece başlatma LTI 1.3 kullanır.

**Gömme, otomatik SSO yerine bir giriş ekranı gösteriyor.** HTML konusu, **Insert Stuff** > **LTI Advantage** yerine FastComments'a işaret eden düz bir `<iframe>` olarak eklenmiş olabilir. Düz iframe'ler LTI başlatmasını atlar ve kullanıcıları genel FastComments sayfasına götürür. Iframe'i silin ve Insert Stuff akışıyla yeniden ekleyin.