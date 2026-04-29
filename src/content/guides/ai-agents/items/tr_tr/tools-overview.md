Bir agent'ın **araçları**, kullanabileceği eylemlerdir. Agent düzenleme formunda bu agent'ın kullanmasına izin verdiğiniz araçları işaretlediğiniz bir **İzin Verilen araç çağrıları** bölümü ve etkili olmadan önce insan onayı gerektirmesi gereken eylemleri işaretlediğiniz bir **Onaylar** bölümü vardır.

Her araç için üç seviye vardır:

- **İzin verilmedi** - agent bunu göremez veya kullanamaz.
- **İzinli, onaysız** - agent doğrudan kullanır. Çalıştırma geçmişine kaydedilir.
- **İzinli, onaylı** - agent çağrısı insan incelemesi için kuyruklanır ve sadece bir insan onayladığında çalışır.

İzin verilmeyen araçlar sessizdir: agent bunları talep edemez ve platform bunları açıkça reddeder. Onayla sınırlanan araçlar her zaman [onay gelen kutusu](#approval-workflow) üzerinden gider.

### Her eylemde denetim izi

Agent'ın yaptığı her eylem kısa bir gerekçe (neden olduğunu açıklayan 1-2 cümle) ve bir güven puanı (0.0-1.0) ile kaydedilir. Her ikisi de [Çalıştırma Detayı Görünümü](#run-detail-view) ve her [onay](#approval-workflow) üzerinde görünür. Bellekte arama, tek yazma-okuma olmayan istisnadır: bir eylem olarak kaydedilmez ve izin listesine bakılmaksızın her zaman kullanılabilir.

### Araç referansı

#### Yorum gönderme

Agent'ın kendisi olarak bir yorum göndermesini sağlar. Yorum agent'ın görüntüleme adı altında herkese açık gösterilir. Karşılama ve özetleyici agent'lar tarafından kullanılır. Geri alınabilir - herhangi bir moderatör kötü bir yorumu kaldırabilir. Genellikle onaysız izin verilir; topluluğunuz her halka açık mesajın insan tarafından incelenmesini gerektiriyorsa bunu onayla sınırlandırın.

#### Bir yorumu düzenleme

Agent'ın kapsam içindeki bir yorumun metnini yeniden yazmasını sağlar. Orijinal metin yorumun denetim günlüğünde korunur. Dar vakalar için ayırın - bir kullanıcının sızdırdığı KİŞİSEL BİLGİLERİ gizleme veya agent'ın kendi önceki yanıtını düzeltme gibi. Görüşleri yeniden yazmak veya üslubu yumuşatmak için değildir. **Onay arkasına almayı kuvvetle düşünün.** Tam sayfa için bkz. [Edit comment](#tool-edit-comment).

#### Yorumlara oy verme

Agent'ın bir yoruma yukarı veya aşağı oy vermesini sağlar. Oy, diğer oylar gibi yorumun oy toplamına katkıda bulunur. Çoğu topluluk botların oy kullanmasını tercih etmez; hiçbir başlangıç şablonunda etkin değildir. İzin verirseniz, oy verme geri alınabilirdir.

#### Bir yorumu sabitle / sabitlemeyi kaldır

Agent'ın bir yorumu sayfanın üstüne sabitlemesini veya zaten sabitlenmiş bir yorumun sabitlemesini kaldırmasını sağlar. Platform bir konu başına bir sabitleme kuralını uygulamaz, bu yüzden sabitleme agent'ına önce önceki sabitlenmiş yorumu kaldırması talimatı verilmelidir. Top Comment Pinner şablonu tarafından kullanılır. Geri alınabilir; genellikle onaysız izin verilir.

#### Bir yorumu kilitle / kilidi aç

Agent'ın bir yorumun altındaki daha fazla yanıtı engellemesini veya yanıtları geri yüklemesini sağlar. Kilitli yorum görünür kalır. Tartışmalı konularda soğuma süresi için yararlıdır, gecikmeli bir kilidin açılması ile eşleştirilebilir. Geri alınabilir ama topluluğunuz tarafından görünür; yüksek riskli topluluklarda onay arkasına almayı düşünün.

#### Spam olarak işaretle / işareti kaldır

Agent'ın bir yorumu spam olarak işaretlemesini (okuyuculardan gizleyip spam sınıflandırıcıya besleyerek) veya bu bayrağı temizlemesini sağlar. Her moderasyon agent'ı için temel araç. Geri alınabilir. Agent'a güven inşa ederken ilk haftalarda onay arkasına almayı kuvvetle düşünün.

#### Bir yorumu onayla / onayı kaldır

Agent'ın beklemede olan bir yorumu okuyuculara göstermesini veya zaten görünür olan bir yorumu gizlemesini sağlar. Yeni yorumları moderatör incelemesi için bekleten tenant'larda en kullanışlı olandır. Görünür bir yorumun onayını kaldırmak yüksek risklidir - her yerde onay arkasına almayı düşünün.

#### Bir yorumu incelendi olarak işaretle

Bir kuyruk durumu aracı: bir yorumu "bir moderatör (veya agent) bunu inceledi" olarak işaretler. Görünürlüğü değiştirmez. Düşük riskli; nadiren onaya tabi tutulur.

#### Bir rozet ver

Agent'ın tenant'ınız için yapılandırdığınız bir kullanıcıya rozet vermesini sağlar. Bir moderatör tarafından geri alınabilir. Nadiren onaya tabi. Bu araç etkinleştirildiğinde agent, tenant'ınızın rozetlerini görebilir ve doğru olanı kendi başına seçebilir, bu yüzden rozet tanımlayıcılarını topluluk yönergelerinize veya başlangıç isteminize yapıştırmanıza gerek yoktur. Hangi davranış için hangi rozetin verileceğini yönlendirmek isterseniz, rozetlere istemde **Görüntü Etiketi** ile referans verin.

#### E-posta gönder

Agent'ın tetikleyicinin kapsamındaki bir yorumun yazarıyla düz metin e-posta göndermesini sağlar. Agent asla alıcının e-posta adresini görmez - bir yorumu seçer ve platform, o yorumcunun gönderdiği adrese teslim eder. Gönderen adresi, yorumun alanı yapılandırılmış bir alanla eşleşiyorsa tenant'ınızın markalı göndericisi (DKIM ile) olur, aksi takdirde platform varsayılanıdır. Seyrek kullanın - e-posta en yüksek sürtünmeli araçtır ve kötü e-postaları geri almak zordur. Onay arkasına almayı kuvvetle düşünün ve onay e-postalarını agent'ın e-posta göndereceği gelen kutusunun sahibine yönlendirin.

#### Agent belleğini kaydet / ara

Tetikleyicinin tetiklendiği kullanıcı hakkında paylaşılan bir not havuzunu okuyan ve yazan eşleşen iki araç. Bellek tenant'ınızdaki tüm agent'lar arasında paylaşılır, bu yüzden bir triage agent'ının notları bir moderatör agent'ının kararlarını bilgilendirir. Arama salt okunurdur ve her zaman erişilebilir; kaydetme nadiren onaya tabi tutulur. Tam tasarım için bkz. [Agent Memory System](#agent-memory-system).

#### Bir kullanıcıyı uyar

Bir kullanıcıya belirli bir yorum hakkında özel bir DM uyarısı gönderir ve uyarıyı atomik olarak agent belleğine kaydeder. Platformun yükseltme politikası bu araç etrafında kuruludur - önce uyar, kullanıcı yeniden suç işlerse banla. `ban_user` aracına göre daha az sıklıkta onaya tabi tutulur, ancak bir agent'ın yaşamının ilk haftalarında onay arkasına almayı düşünün. Tam sayfa için bkz. [Warn user](#tool-warn-user).

#### Bir kullanıcıyı banla

Agent'ın çağırabileceği en sonuç doğurucu araç. Bir kullanıcıyı sabit süreyle banlar, isteğe bağlı olarak bir shadow ban olarak, isteğe bağlı olarak IP'yi de banlayarak, isteğe bağlı olarak da kullanıcının tüm yorumlarını silerek. İki yıkıcı seçenek (IP, tümünü sil) düzenleme formundaki ekstra onaylarla opt-in yapılana kadar modelden tamamen gizlenir. AB bölgesinde, tüm banlar insan onayı gerektirir (bkz. [AB DSA Madde 17 Uyumluluğu](#eu-dsa-compliance)). Her yerde onay arkasına almayı kuvvetle düşünün. Tam sayfa için bkz. [Ban user](#tool-ban-user).

### Ban-aracı alt-seçenekleri

Ban aracı iki yıkıcı seçenek sunar - `delete-all-comments` ve `ban-by-IP` - bunlar düzenleme formunda **Ban options** bölümünde onaylamadığınız sürece modelden tamamen gizlenir. Model parametreyi uydursa bile, platform düzenleme formunda etkinleştirmediğiniz değerleri reddeder. Bkz. [Ban user](#tool-ban-user).