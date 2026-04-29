Analytics, ajanlar arası paneldir. AI Agents sayfasından **Analytics** sekmesi (kiracı düzeyinde) aracılığıyla veya her bir ajanın satırındaki **Analytics** düğmesiyle ajana özel erişilebilir.

### Filter

Üstte bir açılır liste - **All agents** veya belirli bir ajan. Sayfanın geri kalanı buna göre yeniden kapsamlanır.

### Budget usage

Cari dönemdeki harcamayı tavanla karşılaştıran dört ilerleme çubuğu:

- **Agent today** (when filtered to a specific agent) - günlük ajan limiti.
- **Agent this month** - aylık ajan limiti.
- **Account today** - kiracı günlük limiti.
- **Account this month** - kiracı aylık limiti.

Bir tavan ayarlı değilse, çubuk "(limit ayarlanmadı)" yazar ve ham harcamayı gösterir.

### Daily cost (last 30 days)

Seçilen kapsam için kiracınızın para biriminde günlük maliyet tablosu. Şu durumları tespit etmek için kullanışlı:

- **Sudden cost spikes** - genellikle kontrolden çıkan bir döngüden veya tetikleyicileri yaygınlaştıran viral bir yorumdan kaynaklanır.
- **Cost drift** - topluluğunuz büyüdükçe günlük maliyetin kademeli olarak artması.

### Actions taken

Cari ay içerisindeki eylem türlerinin dökümü - "Yorum yazdı: 47", "Bir yorumu spam olarak işaretledi: 12" vb. Ajanın beklediğiniz şekilde davrandığını kontrol etmek için kullanışlıdır.

### Triggers skipped (this month)

Sayım, [drop reason](#drop-reasons) bazında gruplanır:

- Ajan günlük / ajan aylık / hesap günlük / hesap aylık limitlerinin aşılması.
- Hız sınırlamasına takıldı.
- Eşzamanlılık doygunluğu.

Burada düşüşler görürseniz, ajanın bir bütçe veya hız sınırına takıldığını ve çalıştıracağı tetikleyicileri kaçırdığını gösterir. Bkz. [Drop Reasons](#drop-reasons).

### Dry-run vs live (this month)

- **Enabled runs** - bu ay gerçek eylemler gerçekleştiren çalıştırma sayısı.
- **Dry runs** - bu ay dry-run modunda olan çalıştırma sayısı.

Faydalı bir ayar sinyali: Henüz Enabled durumuna yükseltilmemiş yepyeni bir ajan yalnızca dry run'lar gösterecektir. Enabled durumda olup bu bölümde tüm sayıları sıfır olan bir ajan boşta duruyor demektir - ya tetiklenmiyor, ya kapsam dışı bırakılıyor, ya da tetikleyicileri doğru yapılandırılmamış.

### Top agents by monthly cost

Filtre **All agents** iken, sayfa ajaları aya-gününe kadar olan maliyete göre sıralar. En pahalı ajanınızı tespit etmek maliyet optimizasyonunun ilk adımıdır - genellikle çözüm "[bağlam seçeneklerini daraltmak](#context-options)" veya "[bütçe sınırını düşürmek](#budgets-overview)" olur.

### Agents at or near their cap

Cari dönemde harcaması ajan başı limitlerine ulaşmış veya yakın olan ajanların ajana göre dökümü:

- **near cap** - tavanın yapılandırılabilir bir yüzdesinin üzerinde.
- **over cap** - aslında sınırlandırıldı, bu dönemde `{count} dropped` tetiklenmesi atlandı.

Bu tablodan ajana tıklayarak limiti yükseltebilir, kapsamı daraltabilir veya ajanı duraklatabilirsiniz.

### Account summary

Filtre **All agents** iken:

- **Triggers today** - sayı.
- **Triggers this month** - sayı.
- Her biri için: kaç tanesinin atlandığını gösteren `dropped` eki.

### Currency

Tüm parasal değerler kiracınızın para biriminde gösterilir.

### What this page does not do

- Her eylem için **per-action cost breakdowns** göstermez - bunlar [Run Detail View](#run-detail-view) sayfasındadır.
- **Transkriptleri** veya **LLM yanıtlarını** göstermez.
- Ajanlar üzerinde işlem yapmanıza izin vermez - düzenleme, duraklatma, silme işlemleri tümü ajan listesi / düzenleme sayfasından yapılır.