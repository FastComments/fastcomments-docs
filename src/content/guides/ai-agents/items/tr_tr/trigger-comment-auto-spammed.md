---
Bir yorum FastComments'in dahili spam motoru tarafından otomatik olarak spam olarak işaretlendiğinde tetiklenir — bir moderatör tarafından **değil** ve başka bir ajan tarafından da değil.

### Ajanın aldığı bağlam

- Otomatik-spam yapılan yorum.
- Yapılandırmaya bağlı olarak isteğe bağlı konu / kullanıcı geçmişi / sayfa bağlamı.

### Bunu tetikleyen

Platformun spam hattı. Daha fazla ayrıntı için moderasyon rehberindeki [Spam Tespiti](/guide-moderation.html#spam-detection) bölümüne bakın.

### Yaygın kullanım örnekleri

- **İkinci inceleme moderasyonu** - spam motorunun yakalama oranı yüksek ama kesinliği kusurlu; topluluğunuza özgü stil üzerinde eğitilmiş bir ajan yanlış pozitifleri yakalayabilir. Ajan, yanlış sınıflandırılmış bir yorumun işaretini kaldırmak için çağrı yapabilir.
- **Otomatik yasak kaldırma** - tenant'ınız yeni hesapları spam nedeniyle agresifçe yasaklıyorsa, bu tetikleyici üzerindeki bir ajan, bir insan görmeden önce belirgin yanlış pozitifleri inceleyip temizleyebilir.

### Dikkate değer

- Bu tetikleyici moderatör tarafından işaretlenen spam için **tetiklenmez** (bkz: [Tetikleyici: Moderatör Tarafından Spam İşaretlendi](#trigger-moderator-spammed)) ve başka bir ajan tarafından işaretlenen spam için de tetiklenmez.
- Otomatik olarak spam işaretlenen ve daha sonra bir moderatör tarafından Spam Değil olarak işaretlenen bir yorum tetikleyiciyi yeniden tetiklemez.
- Bu tetikleyiciye abone olmak, motorun otomatik spam modunun Moderasyon Ayarları altında etkinleştirildiği tenant'larda en faydalıdır. Aksi takdirde tetikleyici tetiklenmez.

---