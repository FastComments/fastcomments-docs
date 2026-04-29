Çalıştırma Geçmişi, her ajanın çalıştırdığı tüm tetikleyicilerin ajana özel günlük kaydıdır. Ajan listesi sayfasından **Çalıştırmalar** düğmesiyle veya doğrudan `/auth/my-account/ai-agents/{agentId}/runs` adresinden ulaşılabilir.

### Sayfada neler var

Her çalıştırma için bir satır içeren sayfalandırılmış bir tablo:

| Sütun | Anlamı |
|---|---|
| Tarih | Tetikleyicinin tetiklendiği zaman (veya ertelenmiş tetikleyicinin çalıştığı zaman). |
| Durum | **Başladı**, **Başarılı**, veya **Hata**. Eğer çalıştırma dry-run modunda idiyse **Dry Run** rozeti yanında gösterilir. |
| Maliyet | Her çalıştırmanın tenant'ınızın para birimindeki maliyeti. Devam eden (Başladı) çalıştırmalar için boş bırakılır. |
| Eylemler | Çalıştırmadaki araç çağrısı sayısı. |
| Detaylar | [Çalıştırma Detay Görünümü](#run-detail-view) açan bir **Görüntüle** düğmesi. |

### Durumların anlamları

- **Başladı** - çalıştırma ilerlemede veya tamamlanmadan önce son buldu. Olağandışı uzun süre "Başladı" durumunda kalan bir çalıştırma genellikle bir LLM çağrısının zaman aşımını gösterir.
- **Hata** - çalıştırma tamamlandı ancak bir yerde başarısız oldu - LLM çağrısı bir hata döndürdü, bir araç gönderimi başarısız oldu, vb. Detay görünümü spesifik hatayı içerir.
- **Başarılı** - çalıştırma hatasız tamamlandı. Ajan sıfır, bir veya birçok eylem gerçekleştirmiş olabilir.

### Boş durum

Bir ajanın hiç çalıştırması olmadığında sayfada şu gösterilir: "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

Bu son kısım kasıtlıdır - [test çalıştırma akışı](#test-runs-replays), yeni bir ajan için Çalıştırma Geçmişini doldurmanın önerilen yoludur.

### Çalıştırma geçmişi sayfasında olmayanlar

- **Çalıştırılmamış canlı tetikleyiciler** - bütçe, kapsam veya oran sınırlaması nedeniyle düşen bir tetikleyici bu sayfada görünmez. Bunlar [Analizler sayfası](#analytics-page) üzerinde "Atlanan tetikleyiciler" altında görünür.
- **Onaylar** - bu çalıştırmada yapılan eylemler için bekleyen onaylar [onay gelen kutusunda](#approval-workflow) bulunur. Eylem, çalıştırma detay görünümünde **Onay bekleniyor** olarak gösterilir.

### Saklama

Bireysel çalıştırma kayıtları 90 gün saklanır; bu süreden sonra kayıt geçmişten kaybolur. Maliyet ve tetikleyici sayıları uzun vadeli analiz özetlerinde birikmeye devam eder, bu nedenle [Analizler sayfası](#analytics-page) yine de bu zaman diliminin ötesine ait geçmiş toplamları gösterir.

### Yeniden Oynatmalar

Yeniden oynatma tarafından üretilen çalıştırmalar varsayılan olarak canlı çalıştırmalar görünümünden hariç tutulur. Bunları görebileceğiniz yer [Test Çalıştırmaları (Yeniden Oynatmalar)](#test-runs-replays) sayfasıdır.

### Ajanlar arası filtreleme

Çalıştırmalar tablosu ajana özeldir. Ajanlar arası bir çalıştırma görünümü yoktur - çapraz ajan özeti için [Analizler sayfası](#analytics-page) kullanılır. Birden fazla ajan üzerindeki çalıştırmaları incelemeniz gerekiyorsa, [Webhooks](#webhooks-overview) `trigger.succeeded` ve `trigger.failed` olaylarını kendi sisteminize iletirsiniz.

---