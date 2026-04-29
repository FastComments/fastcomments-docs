[Run History](#run-history) içindeki bir satırda **Görüntüle**ye tıklamak her çalışmaya ait detay sayfasını açar. Burada ajanın muhakemesini okur ve aldığı kararları değerlendirirsiniz.

### Top: run summary

- **Agent** - hangi ajanın çalıştığı.
- **When** - zaman damgası.
- **Status** - Started / Success / `Error`, ayrıca uygulanabiliyorsa **Kuru Çalıştırma** rozeti.
- **Cost** - kiracınızın para birimindeki çalışma başına maliyet.
- **Cost per action** - beklemede olmayan işlem sayısına bölünmüş maliyet; olağandışı pahalı çalışmaları tespit etmek için kullanışlıdır.

### Actions taken

Run sırasında yapılan her araç çağrısının sıralı listesi. Her giriş şunları gösterir:

- **Action label** - "Wrote a comment", "Marked a comment as spam", "Banned a user" vb. Etiket, action type enum'undan eşlenir.
- **Reference ID** - etkilenen yorum, kullanıcı veya rozet ID'si, monospaced metin olarak gösterilir (hiperlink değildir).
- **Agent reasoning** - ajanın çağrıyla birlikte sunduğu gerekçe.
- **Confidence** - ajanın kendi değerlendirdiği güven seviyesi, yüzde olarak gösterilir.
- **Pending approval** rozeti - eylem yürütülmek yerine [onay gelen kutusunda](#approval-workflow) sıradaysa gösterilir.

Eğer run sıfır eylem yaptıysa bölüm şöyle yazar: "Bu çalışmada hiçbir eylem gerçekleştirilmedi."

### LLM transcript

Eylemlerin altında, ajanın LLM ile yaptığı konuşmanın tam dökümü:

- **System** - sistem promptu (platform eki + sizin başlangıç isteminiz + topluluk yönergeleri).
- **User** - tetikleyiciyi tanımlayan bağlam mesajı.
- **Assistant** - modelin yanıtları, araç çağrılarını da içerir.
- **Tool** - modele geri beslenen araç sonucu (ör. `search_memory`'nin döndürdüğü).

Uzun mesajlar daraltılabilir; görüntülemek için **Genişlet** / **Daralt**'a tıklayın.

### Reading transcripts

Döküm, ayarlama için en önemli sayfadır. Ajanın aldığı bir karara katılmıyorsanız, tekrar okumak için bakın:

- Modelin **ne gördüğünü** (User bağlam mesajı).
- Modelin **ne kararlaştırdığını** (Assistant araç çağrıları).
- Modelin **neleri değerlendirdiğini** (herhangi bir araç sonucu - ör. ajan gerçekten `search_memory` çağrısı yaptı mı ve yasaklamadan önce bir şey buldu mu).

Model sürekli aynı tür hata yapıyorsa, [başlangıç istemini](#personality-prompt) düzenleyin — veya reddedilen bir onaydan sonra [İstemleri Geliştirme](#refining-prompts) kullanın.

### Action references

Referans ID'leri monospaced metin olarak gösterilir (hiperlink değildir):

- Yorumlar: yorum ID'si.
- Kullanıcılar: kullanıcı ID'si.
- Rozetler: rozet ID'si.

Etkilenen kaydı ilgili moderasyon/yönetici sayfasında aramak için ID'yi kopyalayabilirsiniz.

### Kuru çalıştırmada eksik olanlar

Kuru çalıştırma, aynı eylemleri, gerekçeleri ve güven puanlarını gösterir. Tek fark durum satırındaki **Kuru Çalıştırma** rozeti olur. Yorumlar / kullanıcılar / rozetler için referans ID'leri yine gösterilir — ajan sadece bunları etkilemedi.

### Errors

`Error` durumundaki run'larda detay sayfası altta yatan hata mesajını gösterir. Yaygın hatalar:

- **No LLM API key configured** - kiracı veya platform yapılandırma hatası.
- **LLM call timeout** - LLM sağlayıcısı yavaş veya erişilemezdi.
- **Tool dispatch failure** - ajan hatalı argümanlarla bir araç seçti (ör. artık var olmayan bir yorum ID'si).
- **Budget exhausted mid-run** - çalışmanın ortasında ajanın kotası doldu. Çalışma durduruldu.

Hatalar kısmi eylemleri geri almaz - hatadan önce tamamlanan herhangi bir araç çağrısı kalıcıdır.