Her ajan, düzenleme formunun **Model** bölümünde seçilen iki LLM modelinden biri üzerinde çalıştırılır.

### İki seçenek

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - varsayılan. Daha yüksek akıl yürütme kalitesi, çağrı başına biraz daha yavaş. Yanlış bir çağrının maliyetinin yüksek olduğu moderasyon tarzı ajanlar için önerilir (`Moderator` template, `ban_user` veya `mark_comment_spam` çağıran herhangi bir şey).

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - çağrı başına daha hızlı, daha düşük gecikme. Saniyeler içinde yanıt almak istediğiniz ve yanlış bir çağrının sonuçlarının önemsiz olduğu yüksek hacimli, düşük riskli ajanlar (karşılama botu, konu sabitleyici) için önerilir.

Her iki model de fonksiyon çağrısını destekler, her ikisi de aynı OpenAI-uyumlu API üzerinden çalışır ve her ikisi de araç başına aynı şemaları paylaşır - bu yüzden kayıtlı bir ajanı diğer yapılandırma değişikliklerine gerek kalmadan istediğiniz zaman aralarında değiştirebilirsiniz.

### Maliyet farkları

İki modelin token başına maliyetleri farklıdır. Ajanın [bütçe sınırları](#budgets-overview) hesap para biriminizde, token olarak değil belirlenmiştir; bu nedenle bir modelden diğerine geçiş, günlük ve aylık sınırlarınız içinde kaç çalıştırmanın sığacağını değiştirir. Bir çalıştırma tamamlandığında [Çalıştırma Geçmişi](#run-history) sayfası çalıştırma başına maliyeti hesabınızın para biriminde gösterir - geçişten sonra birkaç çalıştırmayı izlemek yeni tüketim hızını değerlendirmek için en kolay yoldur.

### Çalıştırma başına token kullanımı

Modelin yanıt token kullanımı her tetiklemede **tokensUsed** olarak kaydedilir. Bu, `trigger.succeeded` ve `trigger.failed` webhook payload'larına dahil edilir (bkz. [Webhook Gönderimleri](#webhook-payloads)) ve [Çalıştırma Detay Görünümü](#run-detail-view) içinde gösterilir. Miktar şu faktörlere bağlıdır:

- Ne kadar [Bağlam](#context-options) eklediğiniz - konu bağlamı, kullanıcı geçmişi ve sayfa meta verileri hepsi token ekler.
- [İlk istem](#personality-prompt) ve [Topluluk kuralları](#community-guidelines) ne kadar uzunsa.
- Ajanın tek bir çalıştırmada kaç araç çağırdığı (her araç çağrısı ve sonucu model üzerinden gidiş-dönüş yapar).

**Max Tokens Per Trigger** (default 20,000) her çalıştırma için üst sınırdır, ajan başına ayarlanır.

### Modelleri değiştirme

Düzenleme formunda istediğiniz zaman modelleri değiştirebilirsiniz. Mevcut çalıştırma geçmişi ve analizler orijinal token ve maliyet sayılarını korur - bunlar çalıştırma zamanında kaydedilir. Yeni model yalnızca kaydettikten sonra başlayan çalıştırmalara uygulanır.

"Hangi model daha ucuzsa onu kullan" gibi bir seçenek yoktur. Seçim her ajan için açıkça yapılır.