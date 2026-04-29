Ajanı, ajanın [kapsam](#scope-url-locale) kapsamında olan bir sayfaya her yeni yorum gönderildiğinde tetikler.

### Ajana sağlanan bağlam

- Tam yeni yorum - metin, yazar, oylar, üst yorum ID'si, sayfa URL ID'si.
- İsteğe bağlı: üst yorum ve aynı başlıktaki önceki yanıtlar, eğer [konu bağlamı](#context-options) açıksa.
- İsteğe bağlı: yorumcunun güven faktörü, hesap yaşı, yasağa ilişkin geçmişi ve son yorumları, eğer [kullanıcı geçmişi bağlamı](#context-options) açıksa.
- İsteğe bağlı: sayfa meta verileri, eğer [sayfa bağlamı](#context-options) açıksa.

### Önemli

- Tetikleyici, yorum kalıcı hale getirildikten **sonra** çalışır. Ajan, araç çağrılarında buna doğrudan başvurabilir.
- Aynı kiracı içindeki başka bir ajan tarafından yazılan yorumlar için **çalışmaz**.
- Hem doğrulanmış hem doğrulanmamış yorumlar için tetiklenir. Eğer kiracınız bir yorumun görünür olmadan önce moderatör onayı gerektiriyorsa (moderasyon kılavuzundaki [Onayların Nasıl Çalıştığı](/guide-moderation.html#moderation-approvals) bölümüne bakın), tetikleme yorum oluşturulduğunda, daha sonra onaylandığında değil gerçekleşir. Moderatör botu, inceleme sonrasında yorumları sizin için onaylaması üzere yönlendirilebilir.

### Yaygın kullanımlar

- **Moderasyon** - yorumu topluluk yönergelerine göre kontrol etmek, spam olarak işaretlemek veya ilk kez yazanları uyarmak.
- **Karşılama mesajı** - ancak [Tetikleyici: Yeni Kullanıcının İlk Yorumu](#trigger-new-user-first-comment) genellikle karşılama mesajları için daha uygundur çünkü her kullanıcı için bir kez tetiklenir.
- **Başlık özetleme** - genellikle ajanın çalışmasından önce başlığın sakinleşmesi için bir [tetikleyici gecikmesi](#trigger-deferred-delay) ile birlikte kullanılır.