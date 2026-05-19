Bu sayfa, bir yönetici aracı kaydettikten ve bir dağıtım oluşturduktan sonra FastComments'ın bir Brightspace dersine nasıl ekleneceğini açıklar. Araç henüz kaydedilmediyse önce D2L kayıt kılavuzuna bakın.

<div class="screenshot white-bg">
    <div class="title">Brightspace'te bir birim konusu olarak gömülü FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments Brightspace birimi içinde çalışıyor, dizili yorumları ve bir @-mention seçiciyi gösteriyor" />
</div>

Brightspace iki içerik oluşturma deneyimi sunar: **Classic Content** ve **New Content Experience** (aynı zamanda **Lessons** olarak da adlandırılır). Her ikisi de FastComments'ı sunar, ancak menü yolları farklıdır. Aşağıdaki her bölüm, farklılaştıkları yerleri kapsar.

#### FastComments Araç öğesini Bulma

FastComments aracı, ders içerik düzenleyicisinin içinde iki yerde görünür:

1. Bir modül/birimdeki **Add Existing** düğmesinden (eski Brightspace sürümlerinde **Add Existing Activities** olarak etiketlenir) erişilen etkinlik seçicisinde. Güncel Brightspace sürümlerinde FastComments doğrudan seçicide görünür; eski sürümlerde **External Learning Tools** alt menüsünün altında yer alır. Her iki yol da FastComments'ı bağımsız bir konu olarak ekler.
2. HTML editörü içindeki **Insert Stuff** iletişim kutusunda, **LTI Advantage** altında. Bu, LTI derin bağlantı akışı ile FastComments'ı bir HTML konusu içinde satır içi olarak gömer.

FastComments her iki seçicide de görünmüyorsa, dağıtım dersi barındıran örgüt birimi için etkinleştirilmemiş demektir. Brightspace yöneticinize **Yönetici Araçları** > **Uzantıları Yönet** > **LTI Advantage** > FastComments aracı > **Dağıtımları Görüntüle** yolunu açmasını, dağıtımı açmasını ve dersin org birimini (veya bir üst org birimini) **Org Units** altında eklemesini isteyin.

#### Bir Modüle FastComments Konusu Ekleme

Classic Content:

1. Dersi açın ve gezinme çubuğunda **Content** öğesine tıklayın.
2. Tartışmanın yer alacağı modülü seçin (veya **Add a module** ile bir tane oluşturun).
3. **Add Existing** (eski Brightspace: **Add Existing Activities** > **External Learning Tools**) öğesine tıklayın.
4. Seçicide **FastComments**'a tıklayın. Brightspace modülde bir konu oluşturur ve sizi içerik görünümüne geri getirir.
5. Yeni konuya tıklayın. Satır içi başlık düzenleyicisini kullanarak adını `FastComments Discussion` gibi açıklayıcı bir şeyle değiştirin.

New Content Experience (Lessons):

1. Dersi açın ve **Content** öğesine tıklayın.
2. Tartışmanın yer alacağı birimi ve dersi açın.
3. **Add** > **Existing Activity** öğesine tıklayın ve **FastComments**'ı seçin (eski Brightspace: **External Learning Tools** altında iç içe).
4. Etkinlik derse eklenir.
5. Etkinlik başlığına tıklayarak yeniden adlandırın.

Herhangi bir kullanıcı (öğretim üyesi veya öğrenci) konuyu ilk açtığında, FastComments kaynağa ait bağlantı için tartışma dizisini başlatır. Diziler kaynak bağlantı kimliğine bağlıdır, bu yüzden konunun adını değiştirmek veya taşımak hangi dizinin yüklendiğini etkilemez.

#### Bir HTML Konusunda FastComments'ı Satır İçi Gömme

Okumanın, videonun veya diğer içeriğin altında yorumların aynı konu sayfasında, ayrı bir konu olarak değil de görünmesini istediğinizde bu akışı kullanın.

1. Modülde/derste bir HTML konusu açın veya oluşturun.
2. Brightspace HTML editörünü açmak için **Edit HTML**'ye tıklayın.
3. Yorum dizisinin görünmesini istediğiniz yere imleci yerleştirin.
4. **Insert Stuff** düğmesine (editör araç çubuğundaki puzzle-parça simgesi) tıklayın.
5. Insert Stuff iletişim kutusunda **LTI Advantage** bölümüne kadar kaydırın ve **FastComments**'a tıklayın.
6. FastComments bir derin bağlantı seçici açar. Yerleşimi onaylayın (varsayılan seçenekler içerik tartışmaları için uygundur); **Insert** veya **Continue**'a tıklayın.
7. Brightspace, LTI başlatmasını temsil eden bir yer tutucu bloğu ile HTML editörüne geri döner. Konu üzerinde **Save and Close**'a tıklayın.

Konu yüklendiğinde, Brightspace yer tutucuyu bir iframe ile değiştirir ve iframe LTI aracılığıyla FastComments'ı otomatik başlatır. Öğrenciler tartışma dizisini satır içinde görür.

Tek bir HTML konu birden fazla derin bağlı FastComments gömmesini barındırabilir. Her gömme kendi dizisini alır çünkü her derin bağlantı ayrı bir kaynak bağlantı kimliği üretir.

#### Modül Konusu vs Satır İçi Hızlı Bağlantı

Aşağıdaki durumlarda **modül konusu** yaklaşımını seçin:

- Tartışma, modüldeki adım için birincil etkinlikse.
- Konunun Brightspace içindekiler tablosunda, tamamlanma takibinde ve Sınıf İlerlemesinde görünmesini istiyorsanız.

Aşağıdaki durumlarda **satır içi gömme** yaklaşımını seçin:

- Yorumların aynı sayfadaki diğer içeriğin altında yer almasını istiyorsanız.
- İçindekiler tablosunda ayrı, tamamlanma takibi yapılabilen bir öğe istemiyorsanız.

#### Görünürlük, Taslak ve Serbest Bırakma Koşulları

Yeni bir FastComments konusu varsayılan olarak öğrencilere görünür. Kurulum sırasında gizlemek için:

1. İçerik düzenleyicisinde konu başlığına (Classic) veya etkinlikteki üç nokta menüsüne (New Content Experience) tıklayın.
2. Durumu Classic için **Draft** olarak ayarlayın veya New Content Experience için **Visibility**'yi kapatın.

Taslak konular öğrencilere görünmez. Öğretim üyeleri ve asistanlar onları "Draft" rozeti ile görmeye devam eder.

Konuyu belirli bir grup veya bölüme sınırlamak için:

1. Konuyu açın.
2. Konu başlığı menüsü > **Edit Properties In-place** (Classic) veya **Edit** > **Restrictions** (New Content Experience) öğesine tıklayın.
3. **Release Conditions** altında **Create**'a tıklayın.
4. **Group enrollment** veya **Section enrollment**'ı seçin, grup/bölümü seçin ve kaydedin.

Serbest bırakma koşulları, FastComments'ın kendi rol eşlemesi ile üst üste biner. Konuyu göremeyen öğrenciler LTI başlatması almaz.

#### Öğrencilerin İlk Başlatmada Gördükleri

Bir öğrenci konuya tıkladığında (veya bir gömme içeren bir HTML konusu yüklendiğinde):

1. Brightspace arka planda LTI 1.3 başlatmasını gerçekleştirir.
2. FastComments öğrencinin adını, e-postasını, avatar URL'sini ve LMS rolünü alır ve onları otomatik olarak oturum açtırır. FastComments giriş istemi olmaz.
3. O kaynağa ait yorum dizisi Brightspace iframe'i içinde görüntülenir.

Başlatmadaki rol eşlemesi:

- Brightspace `Administrator` dizide FastComments üzerinde tam yönetim (tam moderasyon, silme, engelleme ve yapılandırma erişimi) sağlayan bir FastComments **admin** olur.
- Brightspace `Instructor` FastComments'ta bir **moderator** olur (pin, gizle, sil, engelle).
- Diğer tüm roller (`Learner`, `TeachingAssistant`, vb.) standart yorumcu olurlar.

Yorumlar öğrencinin Brightspace hesabına atfedilir. Öğrenci Brightspace'te adını veya avatarını düzenlerse, bir sonraki LTI başlatması bu değişikliği senkronize eder.

#### Genel Erişimi Kapatma (Önerilir)

Varsayılan olarak, FastComments yorum verileri genel olarak okunabilir. Birisi bir dizinin URL'sini veya API uç noktasını tahmin edebilirse, Brightspace dışında bile yorumlarını görebilir. Ders tartışmaları için neredeyse kesinlikle görüntülemeyi yalnızca kayıtlı öğrencilerle sınırlandırmak istersiniz.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget özelleştirme sayfanızı</a> açın ve **Require SSO To View Comments** etkin olan bir kural oluşturun, ardından güvenlik düzeyini **Secure SSO** olarak ayarlayın, böylece diziler yalnızca imzalı LTI başlatması aracılığıyla yüklenebilir.

Tam yürütme için, bir kuralı tek bir alan adı veya sayfayla nasıl sınırlandıracağınızı da içeren [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) bölümüne bakın.

#### Iframe Yüksekliği ve Yeniden Boyutlandırma

FastComments her dizin render'ında ve içerik değişikliklerinde (yeni yorum, cevapları genişletme) `org.imsglobal.lti.frameResize` postMessage'ını yayınlar. Brightspace bu mesajı dinler ve iframe yüksekliğini ayarlar, böylece dizi kesilmez ve iç kaydırma çubuğu göstermez.

Iframe sabit kısa bir yükseklikte kalıyorsa:

- Dersin HTTPS üzerinden yüklendiğini doğrulayın. Brightspace'in postMessage dinleyicisi karışık içerikli frameleri reddeder.
- Hiçbir tarayıcı uzantısının postMessage kanalını engellemediğini doğrulayın.
- Bir HTML konusundaki satır içi gömmeler için, çevreleyen HTML iframe'i sabit yükseklikte bir kapsayıcı içine sarmamalıdır. Ebeveyn öğeden herhangi bir inline `style="height: ..."` ifadesini kaldırın.

#### Brightspace'e Özgü Dikkat Edilmesi Gerekenler

**Araç Add Existing seçicisinde görünmüyor.** Dağıtım bu dersin org birimi için etkinleştirilmemiştir. Bir yönetici dağıtımın **Org Units** listesine org birimini (veya bir üstünü) eklemelidir. Araç kaydı tek başına yeterli değildir; dağıtım hangi derslerin aracı gördüğünü sınırlar.

**Başlatmada `deployment_id` uyuşmazlığı.** FastComments bir kayıt için gördüğü ilk `deployment_id`'yi TOFU (Trust On First Use) ile sabitler. Bir yönetici orijinal dağıtımı silip yenisini oluşturursa, yeni dağıtımdan yapılan başlatmalar dağıtım uyuşmazlığı hatasıyla reddedilir. Düzeltme, FastComments'ı yeniden kaydetmektir (yeni kayıt URL'si oluşturun (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan edinin</a>) ve Dinamik Kaydı tekrar çalıştırın); eski yapılandırma kaydı yeni olanla değiştirilir.

**Araç başlatılıyor ama "Invalid LTI launch" gösteriyor.** Ders, dağıtımın kapsadığı kiracı/örgüt yapısının dışında veya dağıtım kayıt sonrası devre dışı bırakılmış olabilir. **Yönetici Araçları** > **Uzantıları Yönet** > **LTI Advantage** > FastComments > **Enabled** geçişini ve dağıtımın org birimi listesini tekrar kontrol edin.

**FastComments içinde isimler ve roller eksik.** Brightspace, NRPS (Names and Role Provisioning Services) iddialarıyla LTI başlatmaları gönderir. Bir ders daha eski bir LTI 1.1 bağlantısından yükseltildiyse, başlatma `name` ve `email` iddialarından yoksun olabilir. FastComments konusunu **Add Existing** ile yeniden ekleyin (eski bağlantıyı migrate etmeyin) ki başlatma LTI 1.3 kullansın.

**Gömme, otomatik SSO yerine bir giriş ekranı gösteriyor.** HTML konusu, **Insert Stuff** > **LTI Advantage** yoluyla değil de doğrudan FastComments'a işaret eden düz bir `<iframe>` olarak eklenmiş olabilir. Düz iframeler LTI başlatmasını atlar ve kullanıcıları halka açık FastComments sayfasına götürür. Iframe'i silin ve Insert Stuff akışı ile yeniden ekleyin.