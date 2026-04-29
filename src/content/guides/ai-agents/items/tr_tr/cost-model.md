Agent maliyeti **token tabanlıdır**. Her LLM çağrısı bir token sayısı döndürür, platform bunu modelin token başına oranını kullanarak USD centlerine dönüştürür ve centler agent ile kiracı bütçelerine fatura edilir.

### Ne ücretlendirilir

- **Tüm LLM çağrıları**, sıfır araç eylemi üreten çağrı da dahil ("agent hiçbir şey yapmamaya karar verdi"). Inferans, hiçbir eylem sonuçlanmasa bile ücretlidir.
- **Dry-run çağrıları**. Dry-run, "eylem yapma, ama yine de LLM'yi çağır" demektir - LLM çağrısı aynı maliyete sahiptir. Bkz. [Dry-Run Modu](#dry-run-mode).
- **Tekrar oynatma (Replay) çağrıları**. Replay'ler, geçmiş yorumlara karşı yapılan dry-run çalıştırmalarıdır. Token maliyeti vardır. Bkz. [Test Runs (Replays)](#test-runs-replays).

### Ne ücretlendirilmez

- **Asla bir LLM çağrısı üretmeyen tetikleyiciler.** LLM'den önce düşürülen vakalar (bütçe aşıldı, hız sınırlaması uygulandı, kapsam uyuşmazlığı, faturalama geçersiz, döngü önleme) sıfır token maliyetine sahiptir. Bkz. [Drop Reasons](#drop-reasons).
- **Araç çağrısı (Tool dispatch).** `pin_comment` veya başka herhangi bir aracı çağırmak kendi başına token maliyeti getirmez - sadece LLM çift yönlü çağrısı maliyetlendirir.
- **`search_memory`.** Bu yalnızca okunur ve kendi başına bir LLM çift yönlü çağrısı üretmez.

### Çalıştırma başına maliyet

Tek bir agent çalıştırması LLM'yi birden çok kez çağırabilir - her araç çağrısının sonucu modele geri beslenir, böylece başka bir aracı çağırabilir veya bitirebilir. Bu yüzden bir çalıştırmadaki `tokensUsed`, o çalıştırmadaki tüm LLM çift yönlü çağrılarının toplamıdır.

Çalıştırma başına token maliyetine en çok katkıda bulunanlar:

- **Uzun [initial prompts](#personality-prompt) ve [community guidelines](#community-guidelines)** - bunlar her çalıştırmaya dahil edilir.
- **[Context options](#context-options)** - konu başlığı bağlamı, kullanıcı geçmişi, sayfa meta verisi. Her biri token ekler.
- **Yorum metninin kendisi** - uzun yorumlar daha fazla maliyetlidir.
- **Tek bir çalıştırmada birden çok araç çağrısı** - her aracın sonuç mesajı modele geri gönderilir.
- **Hafıza okumaları** - `search_memory` en fazla 25 kayıt döndürür (toplam içerik 8000 karakter ile sınırlıdır). Bu baytların çoğu sonraki prompt'a girer.

**Max Tokens Per Trigger** (varsayılan 20.000) her LLM çağrısı için **cevap** boyutunu sınırlar. Girdi boyutunu sınırlamaz.

### Token'dan sente dönüşüm

Platform, kiracı-paketi başına tek bir oran uygular (`flexLLMCostCents` per `flexLLMUnit` tokens). Token başına maliyet paket düzeyindedir, model düzeyinde değil - kullanılabilir her iki model ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) belirli bir pakette aynı oranda faturalandırılır. [Run Detail View](#run-detail-view) bir çalışma tamamlandığında çalıştırma başına maliyeti para biriminizde gösterir.

### Maliyet nerede kaydedilir

Her çalıştırma ham token sayısını ve çalıştırma başına maliyeti kaydeder. Günlük ve aylık toplamlar [Analytics page](#analytics-page)'e toplanır.

### Maliyeti nasıl okunur

- **Çalıştırma başına maliyet**: [Run Detail View](#run-detail-view) -> `Cost` alanı.
- **Günlük / aylık toplam**: [Analytics page](#analytics-page) -> Bütçe kullanımı ve Günlük maliyet grafikleri.
- **Eylem başına maliyet**: ayrıca Run Detail View'da, bir agent'ın araç döngüsü olağandışı uzun olduğunda ayarlama yapmak için faydalıdır.

### Ayrıca bakınız

- [Choosing a Model](#choosing-a-model) - maliyet üzerinde daha büyük etki yapan ayar.
- [Context Options](#context-options) - ek maliyetin geldiği yer.
- [Budgets Overview](#budgets-overview) - kontrolsüz maliyeti önleyen katı sınırlar.

---