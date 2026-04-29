**Şablon Kimliği:** `thread_summarizer`

Thread Özetleyici, uzun bir dizinin sonunda nötr, tek paragraflık bir özet yayınlar. Agent, dizinin sakinleşmesi için 30 dakikalık bir ertelemeye sahiptir.

### Yerleşik başlangıç istemi

[inline-code-attrs-start title = 'Konuşma Özetleyici Şablonu Başlangıç İstemi'; type='text' inline-code-attrs-end]
[inline-code-start]
Nötr konuşma özetleri yayınlarsınız. 5'ten az yorumu olan dizileri özetlemeyin. Daha uzun diziler için, ana pozisyonları, anlaşmazlıkları ve açık soruları tek, kısa bir paragrafta özetleyin. Taraf tutmayın ve editoryal yorum yapmayın. Özeti yayınladıktan sonra sabitleyin. Bu dizide sizden önceki bir özet zaten sabitlenmişse, yeni olanı sabitlemeden önce önceki özetin sabitlenmesini kaldırın.
[inline-code-end]

"editoryal yorum yapmayın" talimatı yük taşıyıcı niteliktedir. Bunun olmaması durumunda model, hesabınızın görüntülenen adı altında kötü görünen "bence" çerçevesine kayma eğilimindedir.

### Tetikleyiciler

- **Yeni yorum eklendi** (`COMMENT_ADD`).
- **Tetikleme gecikmesi**: 30 dakika (1800 saniye). Bkz. [Deferred Triggers](#trigger-deferred-delay).

30 dakikalık gecikme, agent'in bir yorum düştükten yarım saat sonra bir kez çalıştığı ve o anda dizinin nasıl göründüğüne göre hareket ettiği anlamına gelir. Bu, "her yanıta özet" demek değildir — ertelenmiş tetikleyici kuyruğu aynı dizideki birden çok yeni yorum etkinliğini birleştirir, ancak bunları ayrı zaman pencereleri arasında çoğaltmaz. Muhtemelen istemcinize **özel bir kural eklemek** isteyeceksiniz, örneğin "Agent bu diziyi son 24 saatte zaten özetlediyse yeni bir özet gönderme" gibi ve bunu uygulamak için bağlam ile agent'in [hafıza araçlarına](#tools-overview) güvenin.

### İzin verilen araçlar

- [`write_comment`](#tools-overview) - özetin kendisini gönderir.
- [`pin_comment`](#tools-overview) - özeti dizinin en üstünde okuyucuların görmesi için sabitler.
- [`unpin_comment`](#tools-overview) - yeni olanı sabitlemeden önce aynı agent tarafından yapılmış önceki bir özeti sabitlemeyi kaldırır.

Özetleyici, kullanıcıları moderasyona tabi tutamaz veya onlarla etkileşime giremez.

### Özeti sabitleme

Agent yeni bir yorumu `write_comment` ile gönderir, sonra dönen yorum kimliği ile `pin_comment` çağrısı yapar. Aynı diziye karşı sonraki çalışmalarda, istem önce önceki özetini `unpin_comment` ile kaldırmasını söyler — platform kendisi dizi başına tek bir sabitlenmiş yorum kuralını zorlamaz, bu yüzden önceki özeti sabit bırakmak yan yana iki sabitlenmiş özetle sonuçlanır. Agent'in önceki sabitlenmiş özeti görebilmesi için [Context Options](#context-options) içinde "Include parent comment and prior replies in the same thread" seçeneğini işaretleyin.

### Canlıya almadan önce önerilen eklemeler

- **[Context Options](#context-options) içinde "Include parent comment and prior replies in the same thread" seçeneğini işaretleyin.** Bağlamı olmayan bir özetleyici işe yaramaz.
- **Minimum dizi boyutu kuralını ayarlayın.** "5'ten az yorum" istemin varsayılanıdır, ancak yoğun topluluklarda 10-20 daha uygundur. İstemi doğrudan düzenleyin.
- **Sadece uzun biçimli sayfalarda özet istiyorsanız belirli URL desenleriyle kısıtlayın**, duyurular veya ürün sayfaları için değil. Bkz. [Scope: URL and Locale Filters](#scope-url-locale).
- **Maliyetlere dikkat edin.** Özetleme, her çalışmada tüm diziyi okuduğu için en fazla token kullanan şablondur. Etkinleştirmeden önce sıkı bir [günlük bütçe](#budgets-overview) belirleyin.

### Tekrarlayan özetlerin önlenmesi

Agent, [`save_memory`](#tools-overview) ve [`search_memory`](#tools-overview) araçlarına erişime sahiptir - isteminizi genişleterek agent'e "summarized {thread urlId}" notlarını kaydetmesini ve yeniden göndermeden önce bunları kontrol etmesini söyleyebilirsiniz. Hafıza kiracınızdaki tüm agent'lar arasında paylaşılır.

---