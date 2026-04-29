---
Bir yorumun bayrak sayısı yapılandırılmış eşik değerine **tam olarak** ulaştığında tetiklenir.

### Gerekli yapılandırma

- **Flag threshold** - tamsayı >= 1. Tetikleyici, `flagCount === flagThreshold` olduğu anda tetiklenir. Eşiğin üzerine çıkan sonraki bayraklamalarda tekrar tetiklenmez.

Eşik 3 ise ve üç kullanıcı yorumu bayraklarsa, ajan üçüncü bayrakta bir kez tetiklenir. Dördüncü, beşinci veya altıncı bayrak onu **tekrar** tetiklemez.

### Ajanın aldığı bağlam

- Bayraklanmış yorum.
- Yapılandırıldığı şekilde isteğe bağlı konu / kullanıcı geçmişi / sayfa bağlamı.
- Bayrak sayısı yorum bloğunda `Flag Count: N` olarak bulunur.

### Dikkat edilmesi gerekenler

- Tetikleyici yalnızca yorumun platformun bayrak işleme yoluyla eşiği alttan geçmesi durumunda tetiklenir (burada `didIncrement === true`). `flagCount`'ı doğrudan veritabanına yazarak eşiğe ayarlamak tetikleme oluşturmaz; eşikten sonraki bayraklar da tekrar tetiklemez.
- Yorumu kimin bayrakladığını içermez - bayraklar ajan için anonimdir. Bayraklayan kullanıcıları incelemek istiyorsanız, onları kendi verilerinizden getirin.
- Bu tetikleyici için bir tetik gecikmesi (bkz. [Deferred Triggers](#trigger-deferred-delay)) *şiddetle* önerilir - yoğun tartışmalarda bayraklar genellikle toplu olarak gelir ve küçük bir gecikme ajan işlem yapmadan önce tablonun oturmasını sağlar.

### Yaygın kullanım alanları

- **Moderasyon incelemesi** - bayraklanan bir yorum, 'insanlar bunun kötü olabileceğini düşünüyor' sinyalinin kanonik halidir. [Moderator şablonu](#template-moderator) varsayılan olarak bu tetikleyiciye bayrak eşiği 3 olacak şekilde abonedir.
- **Ön moderasyon kuyruğu arttırımı** - ajan ilk bir tarama yapar ve yorumu moderasyona işaretler (with `mark_comment_reviewed`) veya daha ileriye eskale eder.
- **Anti-brigading** - bu tetikleyiciyi [kullanıcı geçmişi bağlamı](#context-options) ile birleştirin ve ajan hareket etmeden önce önceki yasaklar/çoğaltılmış içerik sinyallerini görsün.

### Eşleştirme önerileri

Hem `COMMENT_ADD` hem de `COMMENT_FLAG_THRESHOLD`'e abone olun eğer ilk bakışta bariz vakaları yakalayan ve bayraklar biriktikçe sınırdaki vakaları yeniden değerlendiren bir moderasyon ajanı istiyorsanız. İki olay bağımsız olarak tetiklenir - her ikisine de abone olunduysa ve her ikisi de tetiklenirse ajan iki kez çalışacaktır, fakat ikinci çalışmada artık bayraklanmış durumu görür.

---