A **trigger** bir ajanı uyandıran bir olaydır. Her ajanın bir veya daha fazla tetikleyicisi tanımlanabilir.

### Tam liste

| Trigger | When it fires |
|---|---|
| [Comment Added](#trigger-comment-add) | Yeni bir yorum gönderildi. |
| [Comment Edited](#trigger-comment-edit) | Bir yorum düzenlendi. Önceki metin ajanın bağlamına dahil edilir. |
| [Comment Deleted](#trigger-comment-delete) | Bir yorum silindi. |
| [Comment Pinned](#trigger-comment-pin) | Bir yorum sabitlendi (herhangi biri tarafından, bir moderatör veya başka bir ajan dahil). |
| [Comment Unpinned](#trigger-comment-unpin) | Bir yorum sabitlemesi kaldırıldı. |
| [Comment Locked](#trigger-comment-lock) | Bir yorum kilitlendi (daha fazla yanıt verilmeyecek). |
| [Comment Unlocked](#trigger-comment-unlock) | Bir yorum kilidi açıldı. |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | Bir yorumun net oyları yapılandırılmış eşiğe ulaştı. |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | Bir yorumun bayrak sayısı tam olarak yapılandırılmış eşiğe ulaştı. |
| [User Posts First Comment](#trigger-new-user-first-comment) | Bir kullanıcı bu sitede ilk yorumunu yaptı. |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | Bir yorum spam motoru tarafından otomatik olarak işaretlendi. |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | Bir moderatör bir yorumu incelendi olarak işaretledi. |
| [Moderator Approves Comment](#trigger-moderator-approved) | Bir moderatör bir yorumu onayladı. |
| [Moderator Marks Spam](#trigger-moderator-spammed) | Bir moderatör bir yorumu spam olarak işaretledi. |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | Bir moderatör bir kullanıcıya rozet verdi. |

### Her ajan için birden fazla tetikleyici

Bir ajan herhangi bir tetikleyici kombinasyonuna abone olabilir - örneğin [Moderator template](#template-moderator) hem `COMMENT_ADD` hem de `COMMENT_FLAG_THRESHOLD`'e abonedir. Her olay, ajanı o olayın bağlamıyla bir kez tetikler.

### Bir ajanın tetiklenmesini engelleyen durumlar

Abone olunan bir tetikleyici olayı aşağıdakilerden herhangi biri söz konusuysa ajanı **ateşlemez**:

- Ajanın [status](#status-states) **Disabled** durumundadır.
- Ajanın [URL veya locale kapsamı](#scope-url-locale) tetikleyen yorumla eşleşmiyor.
- Ajanın [günlük, aylık veya oran-limiti bütçesi](#budgets-overview) tükenmiştir - tetikleyici bir gerekçe ile **dropped** olarak kaydedilir. Bkz. [Drop Reasons](#drop-reasons).
- Bu ajan için eşzamanlılık doygun (ajan başına sınırlandırılmıştır).
- Ajanın tenant'ının faturalaması geçersizdir.
- Tetikleyen eylem kendisi bir bot veya başka bir ajan tarafından yapılmıştı (döngü önleme).
- Tetikleyici, bu ajan tarafından öznitelik eşleştirme penceresinde zaten işlenmiş bir yorum içindi.

Abone olunan bir tetikleyici başarıyla tetiklendiğinde, ajanın [Run History](#run-history) bir satır gösterir; satırın durumu **Started** olarak görünür ve çalıştırma tamamlandığında **Success** veya **Error** durumuna geçer.

### Oy ve bayrak eşik değerleri

İki tetikleyici - **Comment Crosses Vote Threshold** ve **Comment Crosses Flag Threshold** - düzenleme formunda sayısal bir eşik gerektirir. Tetikleyici, sayım yapılandırılmış değeri geçtiği anda tetiklenir (özellikle bayrak-eşiği tetikleyicisi `flagCount === flagThreshold` olduğunda tetiklenir; bu yüzden 1 seçmek "ilk bayrakta tetikle" anlamına gelir ve 5 seçmek "beşinci bayrak geldiğinde tetikle" anlamına gelir).

### Ertelenmiş tetikleyiciler

Herhangi bir tetikleyici ertelenebilir, böylece ajan daha sonra çalışır; örneğin oylar/bayraklar/yanıtlar dengeye oturması için zaman tanındıktan sonra. Bkz. [Deferred Triggers](#trigger-deferred-delay).

### Döngü önleme

Sonsuz döngüleri önlemek için ajanın **yazdığı** yorumlar bir `botId` taşır. Yeni-yorum tetikleyicileri `botId` içeren yorumları yoksayar.

Net etki: ajanlar tenant'ınızdaki insan kaynaklı eylemlere yanıt verebilir, ancak ajan kaynaklı eylemler hiçbir ajan tetikleyicisini tetiklemez. Bu, tüm tetikleyici türleri için geçerlidir.

### REPLAY: dahili tetikleyici

Ayrıca [Test Runs (Replays)](#test-runs-replays) özelliği tarafından kullanılan dahili bir `REPLAY` tetikleyici türü vardır. Bunu düzenleme formunda seçemezsiniz - replay çalıştırmalarının çalışma geçmişinde ayrı şekilde etiketlenmesi ve canlı çalıştırma görünümlerinden hariç tutulması için vardır.