Her ajanın harcama limitleri vardır. Bir limit dolduğunda platform ajanı tetiklemeyi durdurur ve dönem dolduğunda yeniden başlatır.

### İki kapsam, iki dönem

Toplam dört limit vardır - iki kapsam (ajan başına, kiracı başına) ve iki dönem (günlük, aylık) ile kesişir.

| Kapsam | Dönem | Nereden ayarlarsınız |
|---|---|---|
| Ajan başına günlük | UTC günü | Ajan düzenleme formu -> **Bütçe** -> **Günlük bütçe** |
| Ajan başına aylık | takvim ayı | Ajan düzenleme formu -> **Bütçe** -> **Aylık bütçe** |
| Kiracı başına günlük | UTC günü | Plan kaynaklı (ayrı bir kullanıcı giriş alanı yok) |
| Kiracı başına aylık | takvim ayı | Plan kaynaklı (ayrı bir kullanıcı giriş alanı yok) |

Bir tetikleme yalnızca **tüm dört limit** izin veriyorsa gönderilir. İlk tükenen limit tetiklemeyi düşüren (drop eden) limit olur.

### Para birimi

Ajan başına bütçeler hesabınızın para biriminde girilir.

### Bir limit dolduğunda ne olur

- Tetikleme `agentDaily` veya `tenantMonthly` gibi bir [düşürme nedeni](#drop-reasons) ile **düşürülmüş** olarak kaydedilir.
- Düşürülen sayısı [Analitik sayfası](#analytics-page) altında "Atlanan tetiklemeler (bu ay)" bölümünde görünür.
- Hiçbir LLM çağrısı yapılmaz; düşürülen tetikleme için herhangi bir token harcanmaz.
- Ajanın durumu değişmez - sadece dönem dolana kadar tetikleme yapamaz.

### Dönem sıfırlanması

- **Günlük** limitler UTC'de gece yarısı sıfırlanır.
- **Aylık** limitler her takvim ayının başında, UTC'de sıfırlanır.

Kullanılmayan bütçe bir sonraki döneme devretmez.

### Sert limitler vs yumuşak uyarılar

Limitler **sert**tir. "Uyarı ile %10 aşma" gibi bir mod yoktur. Limit dolduğunda gönderim durur.

"Yumuşak" olan kısım [Bütçe Uyarıları](#budget-alerts) e-postalarıdır - yapılandırılabilir eşiklerde (varsayılan %80 ve %100) e-posta alırsınız, böylece trafik düşmeden önce limiti yükseltebilirsiniz.

### Mevcut kullanım nereden görülebilir

- [Analitik sayfası](#analytics-page) - ajan bazlı ve kiracı genelinde bütçe kullanımı, limit işaretleriyle.
- Ajan düzenleme formunun **İstatistikler** bölümü.
- Liste görünümü (bekleyen onayların ve son çalıştırmaların sayısı ajan kartında yer alır).

### Bütçe belirleme

Birkaç kural:

- **Yeni bir ajan** - bütçeyi belirleyin. Bir hafta boyunca [Çalıştırma Geçmişi](#run-history) izleyin. Bir çalıştırmanın gözlemlenen maliyeti × beklenen tetikleme hacmine göre ayarlayın.
- **Yüksek hacimli bir ajan** (ör. yoğun bir sitede yeni yorum tetikleyicisi) - kaçan döngüyü yakalayan günlük limittir. Beklenen günlük harcamalarınızın 2-3 katı bir günlük limit seçin, böylece normal yoğun bir gün bunun içinde rahatça kalır.
- **Özetleyici veya bağlam-ağırlıklı ajan** - çalıştırma başına maliyet yüksektir. Kötü bir günün aylığı mahvetmesini önlemek için daha sıkı bir günlük limit belirleyin.

### Yeniden oynatmalar için bütçe atlaması

[Test çalıştırmaları / yeniden oynatmalar](#test-runs-replays) kendi **öz** sert limitlerine tabidir (yeniden oynatma formunda ayarlanır, ajanın günlük/aylık limitlerinden ayrı), VE aynı zamanda ajan ve kiracı limitlerine de tabidir. Hangisi önce dolarsa yeniden oynatmayı durdurur.

### Ayrıca bakınız

- [Bütçe Uyarıları](#budget-alerts) e-posta bildirimleri için.
- [Maliyet Modeli](#cost-model) platformun tokenleri dolara nasıl çevirdiği hakkında.
- [Düşürme Nedenleri](#drop-reasons) bir tetiklemenin çalışmamasının tam neden listesi için.