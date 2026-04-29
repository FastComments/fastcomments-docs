FastComments, AB Dijital Hizmetler Yasası'nın Madde 17'sini AB bölgesindeki kiracılar için uygular: **tamamen otomatik kullanıcı askıya almalarına izin verilmez**.

### Bunun uygulamada ne anlama geldiği

Kiracınız AB bölgesindeyse, acente düzenleme formunda:

- `ban_user` için **Onaylar** onay kutusu **açık olarak kilitlenmiştir** ve işaretini kaldıramazsınız.
- Etiket şöyle okunur: "AB DSA Madde 17: kullanıcı askıya almaları insan incelemesi gerektirir. 'Ban a user' kilitli ve AB bölgesinde tamamen otomatikleştirilemez."
- Onay sütunundaki araç ipucu şöyle der: "AB DSA Madde 17 tarafından açık olarak kilitlenmiştir - AB bölgesinde tamamen otomatik yasaklara izin verilmez."

Ne yapılandırırsanız yapılandırın, bir AB bölgesi kiracısından herhangi bir acente tarafından yapılan her `ban_user` çağrısı insan incelemesi için [approvals inbox](#approval-workflow)'a gider. Yasak, bir insan onaylayana kadar gerçekleşmez.

### Neden bunun platform düzeyinde, prompt düzeyinde değil uygulandığı

Sistem promptları kötü davranan bir model tarafından göz ardı edilebilir veya aşılabilir. Madde 17 uyumu modelin iyi davranışına bırakılamayacak kadar önemlidir; aracının kendisinin zorunlu olarak uyguladığı sunucu tarafı bir kapı olmalıdır. Biz de bunu yapıyoruz.

### Ne onaya gider, ne gitmez

- **`ban_user`**: AB'de her zaman onaya tabidir. Şunları içerir:
  - Görünür yasaklar (`shadowBan: false`).
  - Gizli yasaklar (`shadowBan: true`).
  - `deleteAllUsersComments: true` ile yapılan yasaklar.
  - `banIP: true` ile yapılan yasaklar.
- Tüm yasak türleri, acentenin gerekçesi ve güven seviyesi ile birlikte onay gelen kutusuna düşer; bir insan onaylar veya reddeder.

Diğer acente araçları (`mark_comment_spam`, `warn_user`, `lock_comment` vb.) Madde 17'den etkilenmez. Bunları hâlâ otomatikleştirebilirsiniz. Madde 17 özellikle kullanıcı askıya almaları ile ilgilidir.

### AB dışı kiracılar ne olacak

Kilit AB bölgesinin dışındaki yerlerde uygulanmaz. `ban_user`'ı yine de onayın arkasına koymayı seçebilirsiniz — herhangi bir moderasyon acentesinin ilk haftaları için bunu şiddetle tavsiye ederiz — ancak bu zorunlu değildir.

### Gizli yasaklar

Gizli yasaklar, Madde 17 amaçları için askıya almalar olarak sayılır (kullanıcı gönderi yapabilir ancak içerikleri gizlenir). Bunlar görünür yasaklarla aynı şekilde onaya tabidir.

### Bölge tespiti

Bölge, FastComments dağıtımındaki `REGION` ortam değişkeni tarafından süreç düzeyinde belirlenir (`models/constants.ts` içindeki `isEURegion()` tarafından okunur). Kiracıya özel bir bölge alanı yoktur - kilit, AB'ye dağıtılmış bir örnekteki tüm kiracılara uygulanır. Verilerinizi AB dışı bir dağıtımdan AB dağıtımına taşırsanız, kilit o örnekteki tüm kiracılar için yürürlüğe girer.

### Tüm inceleyiciler yoksa ne olur

Onay, karar verilene kadar gelen kutusunda bekler. Oluşturulmasından 90 gün sonra otomatik olarak süresi dolar. "İnceleyici yok, otomatik karara geç" gibi bir yol yoktur — bu, Madde 17'nin amacını boşa çıkarır.

Topluluğunuz öyle yüksek hacimliyse ki AB yasakları makul bir sürede incelenemiyorsa, şunları düşünün:

- Daha fazla inceleyici eklemek (bkz. [Approval Notifications](#approval-notifications)).
- Acenteyi daha agresif şekilde [`warn_user`](#tool-warn-user) kullanacak şekilde değiştirmek; uyarılar Madde 17'e tabi değildir.
- Acentenin yasaklama iştahını, [community guidelines](#community-guidelines) veya [initial prompt](#personality-prompt) ile sıkılaştırarak düşürmek.

### Ayrıca bakınız

- [Araç: ban_user](#tool-ban-user) — `ban_user`'ın ne yaptığı ve ek opt-in'lerin arkasındaki yıkıcı seçenekler için.
- [Approval Workflow](#approval-workflow) — tam onay yaşam döngüsü için.