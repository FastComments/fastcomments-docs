---
**Şablon Kimliği:** `welcome_greeter`

Welcome Greeter, ilk kez yorum yapanlara sıcak bir şekilde yanıt verir. Bu en düşük riskli şablondur (yıkıcı araç yok) ve canlıya göndermek için iyi bir ilk ajandır.

### Tetikleyiciler

- **Yeni bir kullanıcı bu sitede ilk yorumunu yaptığında** (`NEW_USER_FIRST_COMMENT`).

Bu olay her kullanıcı için tam olarak bir kez tetiklenir, bu yüzden ajan döngüye giremez. Bkz. [Tetikleyici: Yeni Kullanıcının İlk Yorumu](#trigger-new-user-first-comment).

### İzin verilen araçlar

- [`write_comment`](#tools-overview)

Bu tek araçtır - ajan gerçekten moderasyon yapamaz, oy kullanamaz, kullanıcıyı yasaklayamaz veya DM gönderemez.

### Canlıya almadan önce önerilen eklemeler

- **Görünen adı ayarlayın**; davetkar bir şey olsun — "Community Bot", sitenizin maskotu veya marka adınız. Görünen ad, okuyucuların karşılama yanıtına iliştirilmiş olarak gördükleri isimdir.
- **[Context Options](#context-options)** içinde **"Sayfa başlığını, alt başlığını, açıklamayı ve meta etiketlerini dahil et"** seçeneğini işaretleyin. Karşılama botunun yanıtları, sayfanın gerçekte ne hakkında olduğunu referans alabildiğinde belirgin şekilde daha iyi olur.
- **Dil kısıtlamalarını düşünün**; eğer birden fazla dilde hizmet veriyorsanız, yanlış dilde bir karşılama yanıtı kaçırılmış bir yanıttan daha sarsıcıdır. Bkz. [Kapsam: URL ve Yerel Filtreler](#scope-url-locale).

### Neden onay gerekmez

Ajan yalnızca yeni yorumlar yazar ve yalnızca tek seferlik bir tetikleyicide. En kötü senaryo: garip bir selamlama. Engellenmesi gereken yıkıcı bir eylem yok. Çoğu işletmeci, deneme çalışması (dry-run) temiz göründüğünde bunu tamamen onaysız olarak çalıştırır.

---