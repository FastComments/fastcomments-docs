Bir ajanın **araçları**, yapabileceği eylemlerdir. Ajan düzenleme formunda bu ajanın kullanmasına izin verdiğiniz araçları işaretlediğiniz bir **İzin verilen araç çağrıları** bölümü ve etkili olmadan önce insan onayı gerektirmesi gereken eylemleri işaretlediğiniz bir **Onaylar** bölümü vardır.

Herhangi bir araç için üç seviye vardır:

- **Yasaklı** - ajan bunu göremez veya kullanamaz.
- **İzinli, onaysız** - ajan doğrudan kullanır. Çalıştırma geçmişine kaydedilir.
- **İzinli, onaylı** - ajanın çağrısı insan incelemesi için sıraya alınır ve yalnızca bir insan onayladığında çalışır.

Yasaklı araçlar sessizdir: ajan bunları isteyemez ve platform bunları doğrudan reddeder. Onay gerektiren araçlar her zaman [onaylar gelen kutusu](#approval-workflow) üzerinden geçer.

### Her eylemde denetim izi

Ajanın aldığı her eylem, kısa bir gerekçe (nedenini açıklayan 1–2 cümle) ve bir güven skoru (0.0–1.0) ile kaydedilir. Bunların ikisi de [Çalıştırma Detay Görünümü](#run-detail-view) ve her [onay](#approval-workflow) üzerinde görünür. Bellekte arama, tek okuma-istisnasıdır: bir eylem olarak kaydedilmez ve izin listesinden bağımsız olarak her zaman kullanılabilir.

### Araç referansı

#### Yorum gönderme

Ajana kendisi olarak bir yorum göndermesine izin verir. Yorum, ajanın görüntüleme adının altında herkese açık olarak gösterilir. Karşılama ve özetleyici ajanlar tarafından kullanılır. Geri alınabilir - herhangi bir moderatör kötü bir yorumu kaldırabilir. Genellikle onaysız olarak izin verilir; topluluğunuz her halka açık mesajın insan tarafından gözden geçirilmesini istiyorsa onay gerektirin.

#### Yorumlara oy verme

Ajana bir yoruma olumlu veya olumsuz oy verme izni verir. Oy, diğer oylar gibi yorumun oy toplamına sayılır. Çoğu topluluk botların oy kullanmasını tercih etmez; hiçbir başlangıç şablonunda etkin değildir. Eğer izin verirseniz, oy verme geri alınabilir.

#### Bir yorumu sabitle / sabitlemeyi kaldır

Ajana bir yorumu sayfanın üstüne sabitleme veya zaten sabitlenmiş bir yorumun sabitlemesini kaldırma izni verir. Platform, konu başına bir sabitleme kuralını zorlamaz, bu yüzden bir sabitleme ajanı önce önceki sabitlenmiş yorumu kaldırması talimatını almalıdır. Top Comment Pinner şablonu tarafından kullanılır. Geri alınabilir; genellikle onaysız izin verilir.

#### Bir yorumu kilitle / kilit kaldır

Ajana bir yorum altında daha fazla yanıt gelmesini engelleme veya yanıtları geri getirme izni verir. Kilitli yorum görünür kalır. Tartışmalı dizilerde soğuma süresi için faydalıdır; ertelenmiş bir kilit kaldırma ile eşleştirilebilir. Geri alınabilir ancak topluluğunuz tarafından görünür; yüksek riskli topluluklarda onay arkasına koymayı düşünün.

#### Spam olarak işaretle / işaretlemeyi kaldır

Ajana bir yorumu spam olarak işaretleme (okuyuculardan gizleme ve spam sınıflandırıcısına gönderme) veya bu bayrağı temizleme izni verir. Herhangi bir moderasyon ajanı için temel araçtır. Geri alınabilir. Ajanınıza güven inşa edene kadar ilk haftalarda onay arkasına koymayı kuvvetle düşünün.

#### Bir yorumu onayla / onayı kaldır

Ajana bekletilen bir yorumu okuyuculara gösterme veya zaten görünür olanı gizleme izni verir. Yeni yorumları moderatör incelemesi için bekleten tenant'larda en faydalıdır. Görünür bir yorumun onayının kaldırılması yüksek risklidir - her yerde onay arkasına koymayı değerlendirin.

#### Bir yorumu gözden geçirildi olarak işaretle

Bir kuyruk durumu aracı: bir yorumu "bir moderatör (veya ajan) buna baktı" olarak işaretler. Görünürlüğü değiştirmez. Düşük risklidir; nadiren onay arkasına konur.

#### Bir rozet ver

Ajana tenant'ın rozet yapılandırmasından bir kullanıcıya rozet verme izni verir. Bir moderatör tarafından geri alınabilir. Nadiren onay arkasına konur. Ajanın rozet kimliğini bilmesi gerekir; ilgili kimlikleri [topluluk yönergelerinize](#community-guidelines) veya [başlangıç isteminize](#personality-prompt) dahil edin.

#### E-posta gönder

Ajana `noreply@fastcomments.com` adresinden seçtiği bir adrese düz metin e-posta gönderme izni verir. Seyrek kullanın - e-posta en yüksek sürtünme aracıdır ve kötü e-postaları geri almak zordur. Onay arkasına koymayı kuvvetle düşünün ve onay e-postalarını ajanın e-posta göndereceği gelen kutusunun sahiplerine yönlendirin.

#### Ajan belleğini kaydet / ara

Tetikleyici için açılan kullanıcı hakkında ortak bir not havuzunu okuyan ve yazan iki eşleşmiş araç. Bellek tenant'taki tüm ajanlar arasında paylaşıldığından, triage ajanının notları bir moderatör ajanının kararlarına bilgi verir. Arama salt okunurdur ve her zaman kullanılabilir; kaydetme nadiren onay arkasına konur. Tam tasarım için bkz. [Ajan Bellek Sistemi](#agent-memory-system).

#### Bir kullanıcıyı uyar

Belirli bir yorum hakkında kullanıcıya özel bir DM uyarısı gönderir ve uyarıyı atomik olarak ajan belleğine kaydeder. Platformun yükseltme politikası bu araç etrafında kuruludur - önce uyar, kullanıcı yeniden suç işlerse yalnızca yasakla. `ban_user`'dan daha az sıklıkla onay arkasına konur, ancak bir ajanın ilk haftalarında onay arkasına koymayı düşünün. Tam sayfa için bkz. [Kullanıcıyı uyar](#tool-warn-user).

#### Bir kullanıcıyı yasakla

Bir ajanın çağırabileceği en sonuçları ağır araç. Bir kullanıcıyı sabit süreli yasaklar, gölge yasak olarak opsiyonel, isteğe bağlı olarak IP'yi de yasaklama, isteğe bağlı olarak kullanıcının tüm yorumlarını silme. İki yıkıcı seçenek (IP, tümünü sil) düzenleme formunda **Yasak seçenekleri** bölümünde açıkça onaylanana kadar modelden tamamen gizlenir. Model parametreyi halüsine etse bile, platform onaylamadığınız değerleri reddeder. AB bölgesinde tüm yasaklamalar insan onayı gerektirir (bkz. [AB DSA Madde 17 Uyum](#eu-dsa-compliance)). Her yerde onay arkasına koymayı kuvvetle düşünün. Tam sayfa için bkz. [Kullanıcıyı yasakla](#tool-ban-user).

### Yasak aracı alt-seçenekleri

Yasak aracı iki yıkıcı seçenek sunar - delete-all-comments ve ban-by-IP - ve bu seçenekler düzenleme formundaki **Yasak seçenekleri** bölümünden onaylayana kadar modelden tamamen gizlenir. Model parametreyi halüsine etse bile, platform onaylamadığınız değerleri reddeder. Bkz. [Kullanıcıyı yasakla](#tool-ban-user).