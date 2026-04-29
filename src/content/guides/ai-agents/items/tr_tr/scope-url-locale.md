Varsayılan olarak, bir agent tüm tenant'ınızda çalışır — her sayfa, her yerel ayar. Düzenleme formundaki **Scope** ve **Locales** bölümleri bunu daraltmanıza olanak tanır.

### Belirli sayfalara kısıtlama

**Restrict to specific pages** alanı, satır başına bir URL deseni kabul eder; url-pattern glob sözdiziminde olmalıdır. Agent yalnızca sayfa URL'si en az bir desenle eşleşen yorumlarda çalışır. Örnekler:

- `/news/*` - `/news` altındaki herhangi bir sayfa.
- `/forums/*` - `/forums` altındaki herhangi bir sayfa.
- `/blog/2026/*` - `/blog/2026` altındaki herhangi bir sayfa.
- (birden fazla satır birlikte) - herhangi bir desen eşleşiyorsa agent çalışır.

Maksimum: agent başına 50 desen. Desenler geçerli url-pattern glob olmalıdır - form bozuk olanları belirli bir hata ile reddeder.

Alan **boş** olduğunda, agent tenant'taki her sayfada çalışır.

Alan **dolu** olduğunda, agent kapalı başarısız olur: yorumu `urlId` olmayan (ör. sayfa bağlamı olmayan tenant düzeyindeki etkinlikler) herhangi bir tetikleyici atlanır. Bu kasıtlıdır — "/news/* ile sınırlandırılmış" olanın sessizce "her şey"e düşmemesi gerekir.

### Belirli yerel ayarlara kısıtlama

**Restrict to specific locales** çift-körüklü seçim, FastComments yerel ayar ID'lerini kabul eder (`en_us`, `zh_cn`, `de_de`, vb.). Agent yalnızca algılanan yerel ayarı seçili listede olan yorumlarda çalışır.

Algılanan yerel ayar, sayfa yerel ayarına göre gönderim zamanında yorum widget'ı tarafından ayarlanan yorumun `locale` alanından gelir.

Hiçbir yerel ayar seçilmediğinde, agent her yerel ayarda çalışır.

Bir veya daha fazla yerel ayar seçildiğinde, agent kapalı başarısız olur: yorumsuz tetikleyiciler veya `locale` alanı olmayan yorumlardaki tetikleyiciler atlanır.

### Birleşik kapsam

URL ve yerel ayar filtreleri AND ile birlikte çalışır. Bir tetikleyici, agent'ı yalnızca her iki filtre de izin veriyorsa çalıştırır.

Kullanışlı desenler:
- `/news/*` URL deseni + `en_us` yerel ayarı - yalnızca İngilizce haber bölümü.
- URL filtresi yok + birden fazla yerel ayar - tenant genelinde, ancak bu agent'ın istemi yazıldığı diller için.

### Neden bir agent'ı sınırlandırmalısınız

- **Maliyet.** Kapsam daraltma, agent'ın işlemesi gereken tetikleyici hacmini azaltır ve böylece token harcamasını düşürür.
- **Doğruluk.** Teknik makaleler için ayarlanmış bir özetleyici istemi, ürün sayfalarında kötü sonuç verebilir. Kapsam, isteme İngilizce olarak "teknik olmayan sayfaları atla" demekten daha keskin bir araçtır.
- **Yerel ayara özgü davranış.** Yalnızca Almanca yazan bir karşılama mesajı veren agent yalnızca Almanca yerel ayarlı yorumlarda çalışmalıdır. `de_de` yerel ayarı kapsamını [başlangıç istemi](#personality-prompt) içindeki Almanca ton ile birleştirin.

### Kapsamın yapmadıkları

- **Agent slot sayısını** değiştirmez (bkz. [Planlar ve Uygunluk](#plans-and-eligibility)) - sınırlandırılmış bir agent yine bir slot kaplar.
- [Bütçe limitlerini](#budgets-overview) değiştirmez - agent başına günlük ve aylık limitler tüm eşleşen tetikleyiciler için geçerlidir.
- Geçmiş çalışmaları geriye dönük olarak sınırlandırmaz - çalışma geçmişi, agent'ın yaptığı her şeyi gösterir; sonradan daha sıkı bir kapsam belirleseniz bile.