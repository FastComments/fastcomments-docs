Bir moderatör bir kullanıcıya rozet verdiğinde tetiklenir.

### Ajanın aldığı bağlam

- Verilen rozetin **rozet kimliği (badge ID)**.
- **Tetikleyen kullanıcı kimliği** - rozeti veren moderatör.
- Yapılandırıldığı şekilde isteğe bağlı thread / kullanıcı geçmişi / sayfa bağlamı.

Tetikleme yükünde, rozet belirli bir yoruma atıfta bulunularak verilmiş olsa bile fire-site bir `commentId` içermesine **neden olmaz**.

### Bunu kim tetikler

İnsan bir moderatör eylemi.

### Dikkat Çeken Noktalar

- Yalnızca rozet kimliği dahil edilir; ajan rozet meta verilerini (ad, görsel) almaz. Ajanın *hangi* rozetin verildiği hakkında akıl yürütmesi gerekiyorsa, bu bağlamı [başlangıç istemi](#personality-prompt) veya [topluluk kuralları](#community-guidelines) içine gömün.
- Tetikleme kullanıcı başına değil, rozet verme işlemi başına bir kez çalışır. Aynı rozetin bir kullanıcıya iki kez verilmesi, tetiklemeyi iki kez çalıştırır (her verme ayrı bir olaydır).

### Yaygın Kullanımlar

- **Karşılıklı takdir** - belirli bir rozet verildiğinde bir ajan "harika katkınız için teşekkürler" yanıtı gönderebilir.
- [Webhook'lar](#webhooks-overview) aracılığıyla **harici takdir iş akışı** - rozet vermelerini kendi kullanıcı etkileşim sisteminize yansıtın.
- **Hafıza kaydı** - gelecekteki moderasyon ajanlarının kararlarında dikkate alması gereken "bu kullanıcı tanınmış bir katkıda bulunan" notları.