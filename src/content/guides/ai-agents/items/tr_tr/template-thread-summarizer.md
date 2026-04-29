**Şablon Kimliği:** `thread_summarizer`

Thread Summarizer, uzun bir tartışmanın sonunda tarafsız, tek paragraflık bir özet gönderir. Ajanın bakmadan önce tartışmanın sakinleşebilmesi için 30 dakikalık bir erteleme kullanır.

Yerleşik istem, ajana yorumlayıcı bir tutum takınmamasını söyler — bu kritik önemdedir. Bunu yapmazsanız model "benim görüşüme göre" gibi ifadeler kullanma eğilimine girer ve bu, hesabınızın görüntülenen adı altında kötü görünür.

### Tetikleyiciler

- **Yeni yorum eklendi** (`COMMENT_ADD`).
- **Tetikleyici gecikmesi**: 30 dakika (1800 saniye). Bkz. [Ertelenmiş Tetikleyiciler](#trigger-deferred-delay).

30 dakikalık gecikme, ajanın bir yorum düştükten yarım saat sonra sadece bir kez çalıştığı ve o sıradaki tartışmanın nasıl göründüğüne göre hareket ettiği anlamına gelir. Bu, "her yanıta özetleme" demek değildir — ertelenmiş-tetikleyici kuyruğu aynı konuda gelen birden çok yeni-yorum olayını birleştirir, ancak bunları ayrı pencereler arasında çoğaltmaz. Muhtemelen isteminize şu gibi özel bir kural eklemek isteyeceksiniz: "ajan son 24 saat içinde bu konuyu zaten özetlediyse yeni bir özet göndermesin" ve bunu uygulamak için bağlam ile ajanın [hafıza araçları](#tools-overview)'na güvenin.

### İzin verilen araçlar

- [`write_comment`](#tools-overview) - özeti kendisi gönderir.
- [`pin_comment`](#tools-overview) - özeti sabitleyerek okuyucuların konunun en üstünde görmesini sağlar.
- [`unpin_comment`](#tools-overview) - yeni özeti sabitlemeden önce aynı ajan tarafından yapılan önceki bir özeti sabitlenmemiş hale getirir.

Özetleyici, kullanıcıları denetleyemez veya onlarla etkileşime giremez.

### Özeti sabitleme

Ajan, `write_comment` ile yeni bir yorum gönderir, ardından dönen yorum kimliği ile `pin_comment` çağrısı yapar. Aynı konuya sonraki çalıştırmalarda istem, önceki özetini önce `unpin_comment` ile kaldırmasını söyler — platform kendisi her konu için tek bir sabitlenmiş yorum kuralını **zorunlu kılmaz**, bu yüzden önceki özeti sabitli bırakmak yan yana iki sabitlenmiş özetle sonuçlanır. Ajanın önceki sabitlenmiş özeti görebilmesi için [Bağlam Seçenekleri](#context-options) içinde "Include parent comment and prior replies in the same thread" onay kutusunu işaretleyin.

### Canlıya alınmadan önce önerilen eklemeler

- **[Bağlam Seçenekleri](#context-options) içinde "Include parent comment and prior replies in the same thread" onay kutusunu işaretleyin.** Bağlamı olmayan bir özetleyici işe yaramaz.
- **Minimum-konu-boyutu kuralını ayarlayın.** İstem varsayılan olarak "Fewer than 5 comments" kullanır, ancak yoğun topluluklarda 10-20 daha uygundur. İstemi doğrudan düzenleyin.
- **Sadece uzun biçimli sayfalarda özet istiyorsanız belirli URL desenleriyle sınırlandırın**, duyurular veya ürün sayfaları için değil. Bkz. [Kapsam: URL ve Yerel Filtreler](#scope-url-locale).
- **Maliyetleri izleyin.** Özetleme, her çalıştırmada tüm konuyu okuduğu için en çok token kullanan şablondur. Etkinleştirmeden önce sıkı bir [günlük bütçe](#budgets-overview) belirleyin.

### Tekrarlayan özetlerden kaçınma

Ajan, [`save_memory`](#tools-overview) ve [`search_memory`](#tools-overview) araçlarına erişebilir - istemi, "özetlendi {thread urlId}" notlarını kaydetmesi ve yeniden gönderim yapmadan önce bunları kontrol etmesi için genişletebilirsiniz. Bellek, kiracınızdaki tüm ajanlarla paylaşılır.

---