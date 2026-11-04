Agent webhooks, bir agent çalıştırması tamamlandığında veya bir onay durum değiştirdiğinde platformun tetiklediği HTTP callback'lerdir. Bunları, agent etkinliğini kendi sistemlerinize - moderasyon panoları, denetim kayıtları, Slack kanalları, yükseltme araçları - iletmek için kullanın.

[AI Agents sayfasındaki](https://fastcomments.com/auth/my-account/ai-agents) **Webhooks** sekmesi altında yapılandırılır.

### Neler teslim edilir

Dört olay türü:

- **`trigger.succeeded`** - bir agent çalıştırması başarıyla tamamlandı.
- **`trigger.failed`** - bir agent çalıştırması hata verdi.
- **`approval.requested`** - bir eylem insan onayı için kuyruğa alındı.
- **`approval.decided`** - bir onay onaylandı, reddedildi veya yürütme başarısız oldu.

Hangi olayların ne zaman tetiklendiği için [Webhook Events](#webhook-events) bölümüne, her birinin şeması için [Webhook Payloads](#webhook-payloads) bölümüne bakın.

### Neler teslim edilmez

- **Araç başına eylem webhooks.** `pin_comment` çağıran bir çalıştırma, pin için ayrı bir webhook tetiklemez - eylem, çalıştırmanın `trigger.succeeded` payload'ına dahil edilir. Eylem başına teslimat istiyorsanız, trigger payload'ındaki `actions` dizisini ayrıştırın.
- **Düşürülen trigger'lar.** Gönderilmeyen bir trigger (bütçe aşımı, yanlış kapsam) webhook tetiklemez. Düşürmeler yalnızca [Analytics sayfasında](#analytics-page) görünür.
- **Replay ile üretilen trigger'lar.** Test çalıştırmaları webhook tetiklemez. Bkz. [Test Runs (Replays)](#test-runs-replays).

### Yapılandırma

Her webhook için şunları ayarlarsınız:

- **URL** - POST yapılacak HTTPS endpoint'i.
- **Domain** - bu webhook'un geçerli olduğu yorum alan adı (tenant'ınız birden fazla alan adı barındırabilir). `*` tüm alan adlarıyla eşleşir; `*.example.com` bir glob'dur; tam bir alan adı yalnızca kendisiyle eşleşir.
- **Events** - abone olunacak dört olay türünden hangileri.
- **Agents** - "tüm agent'lar" için boş veya filtrelemek için belirli agent ID'lerinin listesi.
- **Method** - POST veya PUT (varsayılan POST).
- **No-retry status codes** - nihai başarısızlık olarak değerlendirilmesi gereken, yeniden denenmeyen HTTP yanıt kodları (örn. 410 Gone, 422 Unprocessable). Bkz. [Webhook Retries](#webhook-retries).

Birden fazla webhook aynı olaya abone olabilir - her biri bağımsız olarak kuyruğa alınır ve kendi URL'sine teslim edilir.

### Alan adına göre eşleştirme

Belirli bir olay, **`domain` alanı yorumun alan adıyla eşleşen her webhook'a** teslim edilir. Eşleştirme basit bir glob kullanır:

- Tam: `example.com` yalnızca `example.com` ile eşleşir.
- Yıldız jokeri: `*` her alan adıyla eşleşir.
- Alt alan adı glob'u: `*.example.com`, `blog.example.com` ve `forum.example.com` ile eşleşir, ancak `example.com` ile eşleşmez.

Payload'daki alan adı, şu kaynaklardan ilk boş olmayan sonuçtur: yorumun `domain` alanı, tenant'ınızın alan adı yapılandırmasına göre ayrıştırılan URL veya `urlId` ile yapılan `Page` araması.

### Agent'a göre filtreleme

**Agents** alanı, bir webhook'un yalnızca belirli agent'lara abone olmasını sağlar. Boş olması "tüm agent'lar" anlamına gelir. Boş olmadığında, webhook yalnızca listedeki agent'lar için tetiklenir.

Bu, moderasyon olayları için bir webhook ve etkileşim olayları için başka bir webhook'a sahip olduğunuzda ve her ikisinin farklı alt sistemlere yönlendirildiği durumlarda kullanışlıdır.

### Test gönderimi

Webhook yapılandırma arayüzünde, gerçek bir olayı beklemeden bağlantıyı, imzalamayı ve endpoint'inizin yanıt kodunu doğrulamanız için URL'ye sahte bir payload gönderen bir **Test send** düğmesi bulunur.

### Teslimat kayıtları

Her teslimat (ve her yeniden deneme), hatta ne olduğunu görebilmeniz için [Webhook Delivery Logs](#webhook-logs) sayfasına kaydedilir.

### İmzalama

Her webhook, tenant'ınızın API secret'ı kullanılarak HMAC-SHA256 ile imzalanır. Bkz. [Webhook Signing](#webhook-signing).

### Uygunluk

Agent webhooks, tenant üzerinde geçerli faturalandırma gerektirir. Mevcut yorum webhook'larınızla aynı imzalama/secret altyapısını kullanır - yorum webhook'larını zaten entegre ettiyseniz (bkz. [Webhooks rehberi](/guide-webhooks.html)), agent webhook entegrasyonu yeni olay türleriyle aynı yapıdadır.