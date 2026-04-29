**Kuru Çalıştırma** her yeni ajanın başladığı güvenlik modudur. Ajan, topluluğunuza dokunan bölüm hariç uçtan uca çalışır.

### Kuru Çalıştırmada neler çalışır

- Tetikleyiciler normal şekilde çalışır.
- Ajanın istemi, [topluluk yönergeleri](#community-guidelines) ve [bağlam](#context-options) oluşturulur.
- LLM çağrılır.
- Model araç çağrılarını seçer ve gerekçeler + güven skorları sağlar.
- Çalıştırma, canlı çalıştırmalardan açıkça ayırt edilebilmesi için bir **Kuru Çalıştırma** rozeti ile kaydedilir.

### Kuru Çalıştırmada neler çalışmaz

- Hiçbir yorum gönderilmez, hiçbir oy kullanılmaz, hiçbir yorum sabitlenmez/serbest bırakılmaz/kilitlenmez/kilit açılmaz.
- Hiçbir yorum spam, onaylanmış veya incelenmiş olarak işaretlenmez.
- Hiçbir kullanıcı yasaklanmaz, uyarılmaz veya rozet verilmez.
- Hiçbir e-posta gönderilmez.
- Hiçbir belleğe yazılmaz. (Evet — bellek de dahil. Kuru-çalıştırma ajanları, varsayımsal kararlarla paylaşılan bellek havuzunu zehirleyemez.)
- Araç eylemleri için hiçbir webhook tetiklenmez. (Tetikleyici düzeyindeki `trigger.succeeded` / `trigger.failed` webhook'ları yine de tetiklenir ve yük, `wasDryRun: true` içerir. Bakınız [Webhook Yükleri](#webhook-payloads).)

### Maliyeti

Kuru Çalıştırma, **Etkin** bir çalıştırmanın yaptığıyla aynı LLM çağrısını yapar. Token'lar ücretlendirilir, [bütçe sınırları](#budgets-overview) uygulanır ve çalıştırmalar ajan başına ve kiracı başına günlük/aylık limitlere sayılır.

Bu maliyet, sadık bir önizleme elde etmenin fiyatıdır. "LLM çağrısını atlama" modu, ajanın nasıl davranacağı hakkında hiçbir sinyal vermez.

### Kuru Çalıştırma sonuçlarını okuma

In [Çalıştırma Geçmişi](#run-history), kuru-çalıştırma çalıştırmaları durum sütununda **Kuru Çalıştırma** rozeti ile işaretlenir. Her bir çalıştırma içindeki eylemler canlı eylemlerle görünüşte aynıdır — aynı araç adı, aynı argümanlar, aynı gerekçe ve güven — ancak bunların hiçbiri gerçekleşmemiştir.

[Analitik sayfası](#analytics-page), aylık bazda "kuru-çalıştırma vs canlı" çalıştırmaları ayırır, böylece token harcamanızın ne kadarının gözleme gittiğini görebilirsiniz.

### Kuru Çalıştırmadan Çıkma

Ajanı düzenleyin ve **Durum**u **Etkin** olarak değiştirin. Bir sonraki tetikleme canlı çalışır.

Ayrıca tersini de yapabilirsiniz — Etkin'i tekrar Kuru Çalıştırma'ya almak — eğer ajan hoşunuza gitmeyen şeyler yapmaya başlarsa. Buna herhangi bir ceza yoktur.

### Yeniden Oynatmalar Kuru Çalıştırmayı Zorunlu Kılar

[Test Çalıştırmaları (Yeniden Oynatmalar)](#test-runs-replays) özelliği, ajanı geçmiş yorumlara karşı **her zaman kuru çalıştırmada** çalıştırır; ajanın kayıtlı durumu ne olursa olsun. Yeniden oynatmalar geçmiş yorumlar üzerinde gerçek işlemler gerçekleştiremez. Bu tasarım gereğidir — yeniden oynatma bir önizleme aracıdır, bir moderasyon aracı değildir.