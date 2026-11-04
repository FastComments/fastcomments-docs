[Run History](#run-history) bölümündeki bir satırda **View** (Görüntüle) düğmesine tıklamak, çalıştırmaya özel detay sayfasını açar. Burada ajanın akıl yürütmesini okur ve kararlarını değerlendirirsiniz.

### Üst kısım: çalıştırma özeti

- **Agent** - hangi ajanın çalıştığı.
- **When** - zaman damgası.
- **Status** - Başlatıldı / Başarılı / Hata, ayrıca uygunsa **Dry Run** rozeti.
- **Cost** - kiracınızın para birimi cinsinden çalıştırma başına maliyet.
- **Cost per action** - beklemeyen eylem sayısına bölünmüş maliyet; alışılmadık derecede pahalı çalıştırmaları tespit etmek için kullanışlıdır.

### Alınan eylemler

Çalıştırmanın yaptığı her araç çağrısının sırayla listesi. Her girdi şunları gösterir:

- **Action label** - "Bir yorum yazdı", "Bir yorumu spam olarak işaretledi", "Bir kullanıcıyı yasakladı" vb. Etiket, eylem tipi enum'undan eşlenir.
- **Reference ID** - etkilenen yorum, kullanıcı veya rozet kimliği, monospace metin olarak gösterilir (köprü değil).
- **Agent reasoning** - ajanın çağrıyla birlikte sağladığı gerekçe.
- **Confidence** - ajanın kendi belirlediği güven düzeyi, yüzde olarak gösterilir.
- **Pending approval** rozeti - eylem yürütülmek yerine [onay gelen kutusunda](#approval-workflow) bekliyorsa.

Çalıştırma sıfır eylem aldıysa, bölüm şunu gösterir: "Bu çalıştırma sırasında hiçbir eylem alınmadı."

### LLM transkripti

Eylemlerin altında, ajanın LLM ile olan konuşmasının tam transkripti:

- **System** - sistem istemi (platform son eki + sizin başlangıç isteminiz + topluluk kuralları).
- **User** - tetikleyiciyi açıklayan bağlam mesajı.
- **Assistant** - araç çağrıları dahil, modelin yanıtları.
- **Tool** - modele geri beslenen araç sonucu (örn. `search_memory` ne döndürdüyse).

Uzun mesajlar daraltılabilir; görüntülemek için **Expand** / **Collapse** düğmelerine tıklayın.

### Transkriptleri okuma

Transkript, ayarlamalar için en önemli sayfadır. Ajan katılmadığınız bir karar verdiğinde, şunları görmek için geriye okuyun:

- Modelin **neyi gördüğü** (User bağlam mesajı).
- Modelin **neye karar verdiği** (Assistant araç çağrıları).
- Modelin **neyi değerlendirdiği** (varsa araç sonuçları - örn. ajan gerçekten `search_memory` çağırdı mı ve yasaklamadan önce bir şey buldu mu).

Model sürekli olarak aynı türde hata yapıyorsa, [başlangıç istemini](#personality-prompt) düzenleyin - veya reddedilmiş bir onaydan [Refining Prompts](#refining-prompts) özelliğini kullanın.

### Eylem referansları

Referans kimlikleri monospace metin olarak gösterilir (köprü değil):

- Yorumlar: yorum kimliği.
- Kullanıcılar: kullanıcı kimliği.
- Rozetler: rozet kimliği.

Kimliği kopyalayıp ilgili moderasyon/yönetim sayfasında etkilenen kaydı arayabilirsiniz.

### Dry-run modunda eksik olan

Dry-run çalıştırmaları **aynı** eylemleri, gerekçeleri ve güven puanlarını gösterir. Tek fark, durum satırındaki **Dry Run** rozetidir. Yorumlar / kullanıcılar / rozetler için referans kimlikleri yine gösterilir - ajan sadece onları etkilememiştir.

### Hatalar

`Error` durumundaki çalıştırmalar için detay sayfası, altta yatan hata mesajını gösterir. Yaygın hatalar:

- **No LLM API key configured** - kiracı veya platform yanlış yapılandırması.
- **LLM call timeout** - LLM sağlayıcısı yavaştı veya kullanılamıyordu.
- **Tool dispatch failure** - ajan hatalı argümanlarla bir araç seçti (örn. artık var olmayan bir yorum kimliği).
- **Budget exhausted mid-run** - çalıştırma sürerken ajanın limiti doldu. Çalıştırma durduruldu.

Hatalar kısmi eylemleri geri almaz - hata öncesinde tamamlanan tüm araç çağrıları geçerli kalır.