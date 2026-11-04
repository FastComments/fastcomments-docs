Analitik, çapraz-ajan panosudur. AI Agents sayfasından **Analytics** sekmesiyle (kiracı genelinde) veya her ajanın satırındaki **Analytics** düğmesiyle ajan başına erişilebilir.

### Filtre

Üstte bir açılır menü - **Tüm ajanlar** veya belirli bir ajan. Sayfanın geri kalanı buna göre yeniden kapsamlanır.

### Bütçe kullanımı

Geçerli dönem harcamasını limite karşı gösteren dört ilerleme çubuğu:

- **Bugün ajan** (belirli bir ajana filtrelenmişse) - günlük ajan limiti.
- **Bu ay ajan** - aylık ajan limiti.
- **Bugün hesap** - kiracı günlük limiti.
- **Bu ay hesap** - kiracı aylık limiti.

Bir limit ayarlanmamışsa, çubuk "(limit ayarlanmadı)" yazar ve ham harcamayı gösterir.

### Günlük maliyet (son 30 gün)

Seçilen kapsam için kiracınızın para biriminde gün başına maliyet tablosu. Şunları tespit etmek için faydalıdır:

- **Ani maliyet artışları** - genellikle kontrolden çıkan bir döngüden veya viral bir yorumun tetikleyicileri çoğaltmasından kaynaklanır.
- **Maliyet kayması** - topluluğunuz büyüdükçe günlük maliyetin kademeli olarak artması.

### Alınan eylemler

Geçerli ay boyunca eylem türlerinin dökümü - "Bir yorum yazdı: 47", "Bir yorumu spam olarak işaretledi: 12" vb. Ajanın beklediğiniz şeyi yapıp yapmadığını kontrol etmek için faydalıdır.

### Atlanan tetikleyiciler (bu ay)

[Atılma nedenine](#drop-reasons) göre gruplandırılmış sayımlar:

- Ajan günlük / ajan aylık / hesap günlük / hesap aylık limiti aşımı.
- Hız sınırına takılma.
- Eşzamanlılık doygunluğu.

Burada atılmalar görüyorsanız, ajanınız bir bütçe veya hız sınırına takılıyor ve normalde çalıştıracağı tetikleyicileri kaçırıyor. Bkz. [Atılma Nedenleri](#drop-reasons).

### Deneme modu vs canlı (bu ay)

- **Etkin çalıştırmalar** - bu ay gerçek eylemler alan çalıştırmaların sayısı.
- **Deneme çalıştırmaları** - bu ay deneme modunda yapılan çalıştırmaların sayısı.

Faydalı bir ayar sinyali: henüz Etkin durumuna yükseltilmemiş yeni bir ajan yalnızca deneme çalıştırmaları gösterecektir. Etkin durumda olup bu bölümde tüm sayaçları sıfır olan bir ajan boşta bekliyor demektir - ya tetiklenmiyordur, ya kapsam dışı bırakılıyordur, ya da tetikleyicileri doğru yapılandırılmamıştır.

### Aylık maliyete göre en üst ajanlar

Filtre **Tüm ajanlar** olduğunda, sayfa ajanları aydan bugüne maliyete göre sıralar. En pahalı ajanınızı tespit etmek maliyet optimizasyonunun ilk adımıdır - cevap genellikle "[bağlam seçeneklerini](#context-options) sıkılaştır" veya "[bütçe limitini](#budgets-overview) düşür" olur.

### Limitinde veya limitine yakın ajanlar

Geçerli dönemde harcaması ajan başına limitinde veya limitine yakın olan ajanların ajan başına dökümü:

- **limitine yakın** - limitin yapılandırılabilir bir yüzdesinin üzerinde.
- **limiti aşmış** - gerçekten sınırlandırılmış, o dönemde `{count} dropped` tetikleyici.

Limiti yükseltmek, kapsamı daraltmak veya duraklatmak için bu tablodan ajana tıklayın.

### Hesap özeti

Filtre **Tüm ajanlar** olduğunda:

- **Bugünkü tetikleyiciler** - sayı.
- **Bu ayki tetikleyiciler** - sayı.
- Her biri için: kaç tanesinin atlandığını gösteren bir `dropped` eki.

### Para birimi

Tüm parasal değerler kiracınızın para biriminde gösterilir.

### Bu sayfanın yapmadıkları

- **Eylem başına maliyet dökümlerini** göstermez - bunlar [Çalıştırma Detay Görünümü](#run-detail-view) üzerindedir.
- **Transkriptleri** veya **LLM yanıtlarını** göstermez.
- Ajanlar üzerinde eylem almanıza izin vermez - düzenleme, duraklatma, silme işlemlerinin tümü ajan listesi / düzenleme sayfasından yapılır.