Bir ajanın **araçları**, yapabileceği eylemlerdir. Ajan düzenleme formunun bu ajanın kullanmasına izin verilen araçları işaretlediğiniz **Allowed tool calls** bölümü ve etkili olmadan önce insan onayı gerektirmesi gereken eylemleri işaretlediğiniz **Approvals** bölümü vardır.

Herhangi bir araç için üç seviye vardır:

- **İzin verilmemiş** - ajan göremez veya kullanamaz.
- **İzinli, onaysız** - ajan doğrudan kullanır. Çalıştırma geçmişine kaydedilir.
- **İzinli, onay gerektirir** - ajanın çağrısı insan incelemesi için kuyruğa alınır ve yalnızca bir insan onayladığında çalışır.

İzin verilmeyen araçlar sessizdir: ajan bunları isteyemez ve platform bunları doğrudan reddeder. Onay kapılı araçlar her zaman [onay gelen kutusu](#approval-workflow) üzerinden gider.

### Her eylem için denetim izi

Ajanın gerçekleştirdiği her eylem, kısa bir gerekçe (nedenini açıklayan 1-2 cümle) ve bir güven puanı (0.0-1.0) ile kaydedilir. Her ikisi de [Run Detail View](#run-detail-view) ve her [onay](#approval-workflow) üzerinde görünür. Bellekte arama tek okuma-yönlü istisnadır: bir eylem olarak kaydedilmez ve izin listesine bakılmaksızın her zaman erişilebilir.

### Araç referansı

#### Yorum gönderme

Ajana, kendisi olarak bir yorum gönderme izni verir. Yorum, ajanın gösterim adı altında herkese açık olarak gösterilir. Karşılama ve özetleme ajanları tarafından kullanılır. Geri alınabilir - herhangi bir moderatör kötü bir yorumu kaldırabilir. Topluluğunuzun her halka açık mesajın insan tarafından incelenmesini gerektirdiği durumlarda onayın arkasına alın.

#### Bir yorumu düzenleme

Ajana, kapsam içi bir yorumun metnini yeniden yazma izni verir. Orijinal metin yorumun denetim günlüğünde korunur. Sınırlı durumlar için saklayın - bir kullanıcının sızdırdığı kişisel olarak tanımlanabilir bilgileri (PII) sansürlemek veya ajanın kendi önceki cevabını düzeltmek gibi. Görüşleri yeniden yazmak veya tonunu yumuşatmak için kullanmayın. Tam sayfa için bkz. [Edit comment](#tool-edit-comment).

#### Yorumlara oy verme

Ajana bir yoruma yukarı veya aşağı oy verme izni verir. Oy, diğer oylar gibi yorumun oy toplamına sayılır. Çoğu topluluk botların oy kullanmasını tercih etmez; hiçbir başlangıç şablonunda etkin değildir. İzin verirseniz, oy verme geri alınabilir.

#### Bir yorumu sabitleme / sabitlemeyi kaldırma

Ajana bir yorumu sayfanın en üstüne sabitleme veya zaten sabitlenmiş bir yorumun sabitlemesini kaldırma izni verir. Platform, konu başına tek-pin kuralını zorlamaz; bu nedenle bir sabitleme ajanına önce önceki sabitlenmiş yorumu kaldırması talimatı verilmelidir. Aynı sayfada zaten hangi yorumların sabitlendiğini keşfetmek için ajan, salt-okunur `get_pinned_comments` aracını çağırabilir (aşağıya bakın). Top Comment Pinner şablonu tarafından kullanılır.

#### Bir yorumu kilitleme / kilidini açma

Ajana bir yorumun altında daha fazla yanıt verilmesini engelleme veya yanıtları geri yükleme izni verir. Kilitli yorum görünür kalır. Hararetli başlıklar için soğuma süresi uygulamak ve ertelenmiş bir kilidin açılması ile eşleştirmek için faydalıdır. Aynı sayfada şu anda hangi yorumların kilitli olduğunu keşfetmek için ajan salt-okunur `get_locked_comments` aracını çağırabilir.

#### Spam olarak işaretleme / işaretini kaldırma

Ajana bir yorumu spam olarak işaretleme (okuyuculardan gizleme ve spam sınıflandırıcısına besleme) veya bu bayrağı temizleme izni verir. Herhangi bir moderasyon ajanı için temel araç. Geri alınabilir.

#### Bir yorumu onaylama / onayını kaldırma

Ajana tutulmuş bir yorumu okuyuculara gösterme veya zaten görünür olan bir yorumu gizleme izni verir. Yeni yorumları moderatör incelemesi için tutan kiracılar (tenant) üzerinde en kullanışlı olanıdır.

#### Bir yorumu incelendi olarak işaretleme

Bir kuyruk durumu aracı: bir yorumu "bir moderatör (veya ajan) bunu inceledi" olarak işaretler. Görünürlüğü değiştirmez. Düşük riskli; nadiren onay arkasına alınır.

#### Rozet verme

Ajana, kiracınız (tenant) için yapılandırdığınız bir rozetin kullanıcıya verilmesini sağlar. Bir moderatör tarafından geri alınabilir. Bu araç etkinleştirildiğinde ajan, kiracınızın rozetlerini görebilir ve doğru olanı kendi başına seçebilir; bu yüzden rozet tanımlayıcılarını topluluk kurallarınıza veya başlangıç isteminize yapıştırmanız gerekmez. Hangi davranış için hangi rozetin verileceğini yönlendirmek için, rozetlere istem içinde **Görüntüleme Etiketi** ile atıfta bulunun.

#### E-posta gönderme

Ajana, tetikleyicinin kapsamındaki bir yorumun yazarı için düz metin bir e-posta gönderme izni verir. Ajan asla alıcının e-posta adresini görmez - bir yorumu seçer ve platform, o yorumu yazarken bıraktıkları adrese teslim eder. from-address, yorumun alanı yapılandırılmış bir alanla eşleştiğinde kiracınızın markalı göndericisi (DKIM ile) olur, aksi takdirde platform varsayılanını kullanır. Seyrek kullanın - e-posta en yüksek sürtünme aracıdır ve kötü gönderilen e-postaları geri almak zordur.

#### Ajan belleğini kaydetme / arama

Tetikleyicinin çalıştığı kullanıcı hakkında paylaşılan not havuzunu okuyan ve yazan eşleştirilmiş iki araç. Bellek, kiracınızdaki tüm ajanlar arasında paylaşılır; bu nedenle bir triyaj ajanın notları bir moderatör ajanın kararlarını bilgilendirir. Arama salt-okunurdur ve her zaman kullanılabilir; kaydetme nadiren onay arkasına alınır. Tam tasarım için bkz. [Agent Memory System](#agent-memory-system).

#### Sabitlenmiş yorumları al / Kilitlenmiş yorumları al

Tetikleyicinin çalıştığı aynı sayfadaki (`urlId`) sabitlenmiş (veya kilitlenmiş) yorumları listeleyen iki salt-okunur keşif aracıdır. Argüman almazlar - sayfa tetik bağlamından okunur, bu yüzden ajan diğer sayfalara yöneltemez. Bir yorum zaten sabitlenmiş veya kilitlenmiş olduğu için üzerinde işlem yapılması gerektiğinde bunları kullanın - tipik olarak `unpin_comment` veya `unlock_comment` çağrısından önce yapılan ilk çağrı veya yeni bir yorumu sabitlemeden önce mevcut olanın önce kaldırılabilmesi için.

Her araç **Allowed tool calls** içinde ayrı ayrı kısıtlanır (yönetici `List pinned comments on the current page` veya `List locked comments on the current page` öğelerini işaretler). Onay arkasına alınamazlar - salt-okunur araçların onay gerektirecek bir yan etkisi yoktur. Bunları çağırmak, çalıştırma geçmişinde bir eylem olarak kaydedilmez; yalnızca ortaya çıkan `unpin_comment` / `unlock_comment` / `pin_comment` çağrısı (varsa) görünür. Liste, çağrı başına en son 20 eşleşme ile sınırlıdır.

Anlaşılması önemli: bu araçlardan biri bir commentId döndürdüğünde, o commentId ajanın çalıştırma başına kapsamına eklenir, böylece takip eden `unpin_comment` / `unlock_comment` çağrısı platformun araç-hedef güvenlik kontrolüne karşı doğrulanır. Keşif aracını önce çağırmadan, ajan tetikleyicinin hemen kapsamı dışında kalan yorumlar üzerinde işlem yapamaz. Bu nedenle bir unpin tarzı ajan tipik olarak her iki aracı da etkinleştirir (ör. `get_pinned_comments` ile `unpin_comment`).

#### Bir kullanıcıyı uyarmak

Bir kullanıcıya belirli bir yorum hakkında özel bir DM uyarısı gönderir ve uyarıyı atomik olarak ajan belleğine kaydeder. Platformun yükseltme politikası bu araç etrafında kuruludur - önce uyar, kullanıcı tekrar suçu işlerse yalnızca o zaman yasakla. Tam sayfa için bkz. [Warn user](#tool-warn-user).

#### Bir kullanıcıyı yasaklama

Bir ajanın çağırabileceği en sonuçları ağır araç. Bir kullanıcıyı sabit süreyle yasaklar, isteğe bağlı olarak gölge yasak (shadow ban), isteğe bağlı olarak IP'yi de yasaklama ve isteğe bağlı olarak kullanıcının tüm yorumlarını silme seçenekleri sunar. İki yıkıcı seçenek (IP, delete-all) düzenleme formunda ekstra onayların arkasında tutulur. AB bölgesinde tüm yasaklamalar insan onayı gerektirir (bkz. [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Tam sayfa için bkz. [Ban user](#tool-ban-user).

### Ban-aracı alt-seçenekleri

Ban aracı iki yıkıcı seçenek sunar - delete-all-comments ve ban-by-IP - ve bunlar, düzenleme formundaki **Ban options** bölümünde bunları etkinleştirene kadar modelden tamamen gizlenir. Model parametreleri hayal etse bile, platform etkinleştirmediğiniz değerleri reddeder. Bkz. [Ban user](#tool-ban-user).

---