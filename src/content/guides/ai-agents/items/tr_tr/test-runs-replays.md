Bir **test çalıştırması** (diğer adıyla **replay**), ajanı geçmiş yorum penceresine karşı **gerçek işlemler yapmadan** çalıştırır. Canlıya geçmeden önce ajan davranışını önizlemenin en hızlı yoludur.

Ajanlar listesinden, her ajanın satırındaki **Test run** düğmesiyle ulaşılabilir.

### Ne yapar

Platform:

1. Seçtiğiniz penceredeki, ajanın kapsamına uyan geçmiş yorumlardan bir örnek seçer.
2. Her yorum için, yorum yeni gönderilmiş gibi ajanı baştan sona çalıştırır - aynı bağlam, aynı LLM çağrısı, aynı araç seçimi, aynı gerekçeler ve güven skoru.
3. Her çalıştırmayı dry-run olarak kaydeder; hangi replay'den geldiği etiketlenir, böylece gruplanmış kalır ve canlı çalıştırma görünümlerinden hariç tutulur.
4. Ajanın verdiği kararı, yorumla gerçekte olanlar ile **karşılaştırır** - daha sonra onaylandı mı, spam olarak işaretlendi mi, silindi mi, spam motoru tarafından engellendi mi vb.

Sonuç, yorum başına bir farktır: "Replay ajanı bunu spam olarak işaretlerdi, fakat yorum şu anda onaylı ve temiz."

### Yapılandırma

Test-run sayfasının tek bir girişi vardır:

- **Değerlendirilecek geçmiş yorum gün sayısı** - 1 ile 90 arasında bir sayısal `days` alanı. Daha eski yorumlar uygun değildir.

Örnek boyutu ve sert tavan UI'da **gösterilmez** - her ikisi de plan başına uygulanan sunucu tarafı varsayılanlarıdır. Sayfa bilgi alanları gösterir:

- **Penceredeki eşleşen yorumlar** - kaç yorumun değerlendirileceği.
- **Bu pencereden en fazla N yorum işlenecektir** - sunucu tarafı tavanı göz önüne alındığında etkin örnek boyutu.
- **Tahmini maliyet** - kiracı para biriminizde.

### Oran sınırlaması

Her kullanıcı, **24 saat içinde 10 test çalıştırması** ile sınırlıdır (anahtar üzerinden oran sınırlaması: `replay-create:${requestedBy}`). Düğme, limite ulaştığınızda bir araç ipucu gösterir ("Son 24 saatte 10 test çalıştırmasına ulaştınız.").

### Eşzamanlılık

Her ajan için aynı anda yalnızca bir replay aktif olabilir. Bir replay devam ederken ikinci bir replay başlatmaya çalışmak sizi o anda devam eden replay'e yönlendirir.

### Sonuçları okuma

Replay tamamlandığında, sonuç sayfası sekmeler gösterir:

- **Deltas** (varsayılan aktif) - replay ajanının kararı gerçeğiyle farklı. (En ilginç olan - "ajan bu yorumu spam olarak işaretlerdi, ama yorum onaylanmış ve sorun yok".)
- **Matches** - replay ajanının kararı gerçeğe uyuyor. (Rahatlatıcı - ajan gerçekle uyuşuyor.)
- **No action** - replay ajanı hiçbir şey yapmamaya karar verdi. (Bazen doğru cevap; bazen ajan bir şeyi kaçırmış demektir.)
- **All** - sınıflandırmadan bağımsız tüm sonuçlar.

Her sekmedeki her yorum için:

- **Önceki sonuç** - gerçekte ne olduğu sınıflaması: **POSITIVE**, **NEGATIVE**, veya **INDETERMINATE**, birlikte **Delil** ("Yorum {date} tarihinde silindi olarak işaretlendi", "Motor: bayes" vb.).
- **Replay ajanı yapardı** - ajanın seçtiği eylem.
- **Neden** - gerekçe.
- **Güven** - yüzde olarak gösterilir.

### Neden replay'ler dry-run zorunlu kılar

Dört ay önce silinmiş bir yoruma karşı yapılan bir replay, onu geriye dönük olarak silmemelidir - zaten silinmiştir. Ajanın şimdi onaylamak istediği bir yoruma karşı yapılan bir replay, yorumun mevcut durumunu değiştirmemelidir. Replay bir önizleme aracıdır. Dry-run'ı zorunlu kılmak, herhangi bir geçmiş penceresine karşı replay çalıştırmayı güvenli kılan şeydir.

### Tekrarlanabilirlik

Replay'ler, replay başlatıldığı anda ajanın yapılandırmasını dondurur. Ajan üzerinde sonraki düzenlemeler replay'in sonuçlarını değiştirmez - sonuç sayfası, o sürümün ne yapacağını gösteren sabit bir kayıt olarak kalır.

### Bütçeler replay'i ne zaman durdurur

Replay'ler şunlara tabidir:

- Replay formunda belirlenen kendi **sert tavanına**.
- Ajanın günlük ve aylık **bütçe tavanlarına**.
- Kiracının günlük ve aylık **bütçe tavanlarına**.

İlk karşılaşılan tavan, replay'i belirli bir hata kodu ile sonlandırır. Sonlandırmadan önce üretilen herhangi bir yorum başına sonuç [Çalıştırma Geçmişi](#run-history) içinde saklanır.

### Replay'ler nasıl çalışır

Replay'ler arka planda, senkron olmayan şekilde çalışır. "Start test run"a tıkladıktan sonra, replay sıraya alınır ve bir worker onu alır. Uzun bir replay birkaç dakika sürebilir. Sonuç sayfası ilerlemeyi (işlenen sayısı, şu ana kadar harcama) ara ara sorgular ve gösterir.

Bir worker replay ortasında ölürse, platform replay'i otomatik olarak yeniden sıraya alır, böylece sonraki çalıştırmada devam eder. Kısa bir kesinti asla bir replay'i sahipsiz bırakmaz.

### Replay'in yapmadıkları

- **[trigger delays](#trigger-deferred-delay)**'i dikkate almaz. Replay'ler hemen çalışır, 30 dakika sonra değil.
- **Belleğe yazmaz.** Replay ajanları notları bellek olarak kaydetmez, mantıkları normalde kaydedecek olsa bile.
- **Webhook tetiklemez.** Replay tarafından üretilen tetikleyiciler `trigger.succeeded` / `trigger.failed` webhook olaylarını oluşturmaz.
- **Zaten replay yapılmış yorumları hariç tutmaz.** Aynı pencereye karşı ikinci bir replay çalıştırmak aynı yorumları kapsar.

### Ayrıca bakınız

- [Refining Prompts](#refining-prompts) - replay'lerle iyi eşleşen yinelemeli düzenleme iş akışı.
- [Dry-Run Mode](#dry-run-mode) - aynı fikir, canlı trafik karşısında.