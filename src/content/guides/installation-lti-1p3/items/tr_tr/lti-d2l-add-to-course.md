This page covers adding FastComments to a Brightspace course after an administrator has registered the tool and created a deployment. If the tool is not registered yet, see the D2L registration guide first.

<div class="screenshot white-bg">
    <div class="title">Brightspace içinde bir birim konusu olarak gömülü FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace iki içerik oluşturma deneyimi sunar: **Klasik İçerik** ve **Yeni İçerik Deneyimi** (aynı zamanda **Dersler** olarak da adlandırılır). Her ikisi de FastComments'i sunar, ancak menü yolları farklıdır. Aşağıdaki her bölüm, farklılaştıkları yerlerde her ikisini de kapsar.

#### FastComments Aracını Bulma

FastComments aracı bir kurs içerik düzenleyicisi içinde iki yerde görünür:

1. Bir modül/birimdeki **Add Existing** düğmesinden erişilen etkinlik seçicisinde (eski Brightspace sürümlerinde **Add Existing Activities** olarak etiketlenmiştir). Güncel Brightspace sürümlerinde FastComments doğrudan seçicide görünür; eski sürümlerde **External Learning Tools** alt menüsü altında yer alır. Her iki yol da FastComments'i bağımsız bir konu olarak ekler.
2. HTML düzenleyicisi içindeki **Insert Stuff** iletişim kutusunda, **LTI Advantage** altında. Bu, LTI derin bağlantı akışı aracılığıyla FastComments'i bir HTML konusu içinde satır içi gömme olarak ekler.

FastComments her iki seçicide de görünmüyorsa, dağıtım kursu barındıran org birimi için etkinleştirilmemiş demektir. Brightspace yöneticinizden **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments aracı > **View Deployments** öğesini açmasını, dağıtımı açmasını ve kursun org birimini (veya bir üst org birimini) **Org Units** altında eklemesini isteyin.

#### Bir Modüle FastComments Konusu Ekleme

Klasik İçerik:

1. Kursu açın ve gezinme çubuğunda **Content**'e tıklayın.
2. Tartışmanın yer alacağı modülü seçin (veya **Add a module** ile bir tane oluşturun).
3. **Add Existing**'e tıklayın (eski Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Seçicide **FastComments**'e tıklayın. Brightspace modülde bir konu oluşturur ve sizi içerik görünümüne geri döndürür.
5. Yeni konuya tıklayın. Satır içi başlık düzenleyicisini kullanarak adını `FastComments Discussion` gibi açıklayıcı bir şeye değiştirin.

Yeni İçerik Deneyimi (Dersler):

1. Kursu açın ve **Content**'e tıklayın.
2. Tartışmanın yer alacağı birim ve dersi açın.
3. **Add** > **Existing Activity**'ye tıklayın ve **FastComments**'i seçin (eski Brightspace: **External Learning Tools** altında yer alır).
4. Etkinlik derse eklenir.
5. Etkinlik başlığına tıklayarak yeniden adlandırın.

Herhangi bir kullanıcı (eğitmen veya öğrenci) konuyu ilk kez açtığında, FastComments o kaynak bağlantısı için diziyi başlatır. Dizi kaynak bağlantı kimliğine bağlıdır, bu yüzden konunun adını değiştirmek veya taşımak hangi dizinin yükleneceğini etkilemez.

#### FastComments'i Bir HTML Konusunda Satır İçi Olarak Gömme

Okuma, video veya diğer içeriğin altında, ayrı bir konu olarak değil aynı konu sayfası içinde yorumların görünmesini istediğinizde bu akışı kullanın.

1. Modül/ders içinde bir HTML konusu açın veya oluşturun.
2. Brightspace HTML düzenleyicisini açmak için **Edit HTML**'e tıklayın.
3. Yorum dizisinin görünmesi gereken yere imleci yerleştirin.
4. **Insert Stuff** düğmesine tıklayın (düzenleyici araç çubuğundaki puzzle-parça simgesi).
5. Insert Stuff iletişim kutusunda **LTI Advantage** bölümüne kaydırın ve **FastComments**'e tıklayın.
6. FastComments bir derin bağlantı seçim penceresi açar. Yerleşimi onaylayın (varsayılan seçenekler içerik tartışmaları için uygundur); **Insert** veya **Continue**'a tıklayın.
7. Brightspace, LTI başlatmasını temsil eden bir yer tutucu bloğu ile HTML düzenleyicisine geri döner. Konuda **Save and Close**'a tıklayın.

Konu yüklendiğinde Brightspace yer tutucuyu LTI ile FastComments'i otomatik başlatan bir iframe ile değiştirir. Öğrenciler tartışma dizisini satır içinde görür.

Tek bir HTML konu birden fazla derin bağlantılı FastComments gömme barındırabilir. Her gömme kendi dizesini alır çünkü her derin bağlantı farklı bir kaynak bağlantısı kimliği üretir.

#### Modül Konusu vs Satır İçi Hızlı Bağlantı

Aşağıdaki durumlarda **modül konusu** yaklaşımını seçin:

- Tartışma modüldeki o adım için birincil etkinlikse.
- Konunun Brightspace'in içerik tablosunda, tamamlama takibinde ve Sınıf İlerleme'de görünmesini istiyorsanız.

Aşağıdaki durumlarda **satır içi gömme** yaklaşımını seçin:

- Yorumların aynı sayfadaki diğer içeriğin altında görünmesini istiyorsanız.
- İçerik tablosunda ayrı ve tamamlama takibi yapılabilen bir öğe olmasını istemiyorsanız.

#### Görünürlük, Taslak ve Serbest Bırakma Koşulları

Yeni bir FastComments konusu varsayılan olarak öğrencilere görünür. Kurulum yaparken gizlemek için:

1. İçerik düzenleyicisinde konu başlığına (Klasik) veya etkinliğin üç nokta menüsüne (Yeni İçerik Deneyimi) tıklayın.
2. Durumu **Draft** olarak ayarlayın (Klasik) veya **Visibility**'yi kapatın (Yeni İçerik Deneyimi).

Taslak konular öğrencilere görünmez. Eğitmenler ve öğretim asistanları yine de bunları "Draft" rozetiyle görür.

Konuyu belirli bir grup veya bölüme sınırlamak için:

1. Konuyu açın.
2. Konu başlığı menüsüne tıklayın > **Edit Properties In-place** (Klasik) veya **Edit** > **Restrictions** (Yeni İçerik Deneyimi).
3. **Release Conditions** altında **Create**'e tıklayın.
4. **Group enrollment** veya **Section enrollment**'ı seçin, grup/bölümü seçin ve kaydedin.

Serbest bırakma koşulları FastComments'in kendi rol eşlemesi ile üst üste binmektedir. Konuyu göremeyen öğrenciler LTI başlatmasını alamazlar.

#### Öğrencilerin İlk Başlatmada Gördükleri

Bir öğrenci konuya tıkladığında (veya gömme içeren bir HTML konusunu yüklediğinde):

1. Brightspace arka planda LTI 1.3 başlatmasını gerçekleştirir.
2. FastComments öğrencinin adını, e-postasını, avatar URL'sini ve LMS rolünü alır ve onları otomatik olarak oturum açtırır. FastComments giriş istemi görünmez.
3. O kaynak bağlantısına ait yorum dizisi Brightspace iframe'i içinde görüntülenir.

Başlatmada rol eşlemesi:

- Brightspace `Administrator` dizide FastComments içinde bir **admin** olur (tam yönetim, silme, yasaklama ve yapılandırma erişimi).
- Brightspace `Instructor` bir FastComments **moderator** olur (sabitleme, gizleme, silme, yasaklama).
- Diğer tüm roller (`Learner`, `TeachingAssistant`, vb.) standart yorumcu olur.

Yorumlar öğrencinin Brightspace hesabına atfedilir. Öğrenci Brightspace içinde adını veya avatarını düzenlerse, sonraki LTI başlatması bu değişikliği senkronize eder.

#### Iframe Yüksekliği ve Yeniden Boyutlandırma

FastComments her dizi render'ında ve içerik değişikliklerinde (yeni yorum, cevapları genişletme) `org.imsglobal.lti.frameResize` postMessage'ını gönderir. Brightspace bu mesajı dinler ve iframe yüksekliğini, dizinin kesilmemesi ve iç kaydırma çubuğu göstermemesi için ayarlar.

Eğer iframe sabit kısa bir yükseklikte kalıyorsa:

- Kursun HTTPS üzerinden yüklendiğini doğrulayın. Brightspace'in postMessage dinleyicisi karma içerikli çerçeveleri reddeder.
- Hiçbir tarayıcı uzantısının postMessage kanalını engellemediğini doğrulayın.
- Bir HTML konusundaki satır içi gömmeler için, çevreleyen HTML iframe'i sabit yükseklikte bir kapsayıcıya sarmamalıdır. Üst öğeden herhangi bir inline `style="height: ..."` kaldırın.

#### Brightspace'e Özgü Tuzaklar

**Araç Add Existing seçicide görünmüyor.** Dağıtım bu kursun org birimi için etkinleştirilmemiştir. Bir yönetici, org birimini (veya bir üstünü) dağıtımın **Org Units** listesine eklemelidir. Araç kaydı tek başına yeterli değildir; dağıtım hangi kursların aracı gördüğünü belirler.

**Başlatmada `deployment_id` uyuşmazlığı.** FastComments, kayıt için gördüğü ilk `deployment_id` değerini TOFU ile sabitler. Bir yönetici orijinal dağıtımı silip yenisini oluşturursa, yeni dağıtımdan yapılan başlatmalar dağıtım uyuşmazlığı hatasıyla reddedilir. Çözüm, FastComments'i yeniden kaydetmektir (yeni bir kayıt URL'si oluşturun ve Dinamik Kayıt işlemini yeniden yürütün); eski yapılandırma kaydı değiştirilir.

**Araç başlatılıyor ama "Invalid LTI launch" gösteriyor.** Kurs, dağıtımın kapsadığı tenant/org yapısının dışında olabilir veya dağıtım kayıt sonrası devre dışı bırakılmış olabilir. **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** anahtarını ve dağıtımın org birimi listesini tekrar kontrol edin.

**İsimler ve roller FastComments içinde eksik.** Brightspace LTI başlatmalarını Names and Role Provisioning Services (NRPS) iddiaları ile gönderir. Eğer bir kurs eski bir LTI 1.1 bağlantısından yükseltildiyse, başlatma `name` ve `email` iddialarından yoksun olabilir. FastComments konusunu **Add Existing** ile yeniden ekleyin (eski bağlantıyı taşımayın) böylece başlatma LTI 1.3 kullanır.

**Gömme otomatik SSO yerine giriş ekranı gösteriyor.** HTML konusu **Insert Stuff** > **LTI Advantage** aracılığıyla değil, FastComments'e işaret eden düz bir `<iframe>` olarak eklendi. Düz iframeler LTI başlatmasını atlar ve kullanıcıları genel erişimli FastComments sayfasına yönlendirir. iframe'i silin ve Insert Stuff akışı ile yeniden ekleyin.