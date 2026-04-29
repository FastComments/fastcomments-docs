**Template ID:** `welcome_greeter`

Welcome Greeter, ilk kez yorum yapan kullanıcılara sıcak bir karşılama yanıtı verir. Yıkıcı araç içermediği için en düşük riskli şablondur ve canlıya gönderebileceğiniz iyi bir ilk ajandır.

### Yerleşik başlangıç istemi

[inline-code-attrs-start title = 'Welcome Greeter Şablonu Başlangıç İstemi'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Tetikleyiciler

- **Yeni kullanıcı bu sitede ilk yorumunu gönderir** (`NEW_USER_FIRST_COMMENT`).

Bu etkinlik her kullanıcı için tam olarak bir kez tetiklenir, bu yüzden ajan döngüye giremez. Bkz. [Tetikleyici: Yeni Kullanıcının İlk Yorumu](#trigger-new-user-first-comment).

### İzin verilen araçlar

- [`write_comment`](#tools-overview)

Bu tek araçtır - ajan gerçekten moderasyon yapamaz, oy veremez, yasaklayamaz veya özel mesaj gönderemez.

### Canlıya almadan önce önerilen eklemeler

- **Görünen adı** davetkar bir şey olarak ayarlayın - "Community Bot", sitenizin maskotu veya marka adınız. Görünen ad, okuyucuların karşılama yanıtına iliştirilmiş olarak gördüğü şeydir.
- **"Sayfa başlığını, alt başlığını, açıklamasını ve meta etiketlerini dahil et"** seçeneğini [Bağlam Seçenekleri](#context-options) içinde işaretleyin. Karşılama botunun yanıtları, sayfanın gerçekte ne hakkında olduğunu referans alabildiğinde belirgin şekilde daha iyi olur.
- **Yerel ayar kısıtlamalarını düşünün** eğer birden fazla dilde hizmet veriyorsanız. Yanlış dilde bir karşılama yanıtı, kaçırılmış bir yanıttan daha sarsıcıdır. Bkz. [Kapsam: URL ve Yerel Ayar Filtreleri](#scope-url-locale).

### Neden onaylara gerek yok

Ajan yalnızca yeni yorumlar yazar ve yalnızca tek seferlik bir tetikleyici ile çalışır. En kötü senaryo: garip bir karşılama. Engellenmesi gereken yıkıcı bir eylem yok. Çoğu işletmeci, deneme çalışması temiz göründükten sonra bunu hiçbir onay olmadan çalıştırır.