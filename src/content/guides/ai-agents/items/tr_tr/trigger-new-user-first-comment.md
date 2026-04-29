Bir kullanıcı bu sitede (tenant'iniz) ilk yorumunu yaptığında tetiklenir. Bu **kullanıcı başına bir kez** gerçekleşir - aynı kullanıcının sonraki yorumları tetikleyiciyi yeniden çalıştırmaz.

### Ajanın aldığı bağlam

- Yeni yorum.
- Yapılandırıldığı şekilde isteğe bağlı başlık / kullanıcı geçmişi / sayfa bağlamı.

Kullanıcı geçmişi bağlamı açık olduğunda, kullanıcının son yorumlar listesi elbette boş olacaktır (veya yalnızca bu yorumu içerecektir), ancak güven faktörü ve hesap yaşı doldurulur.

### Önemli

- "Bu sitedeki ilk yorum" **tenant** ile sınırlıdır, FastComments genelinde site çapında değildir. Diğer FastComments sitelerinde yorumları olan bir kullanıcı, sizin sitenize ilk kez yorum yaptığında yine de bu tetikleyiciyi çalıştırır.
- Tetikleyici yalnızca bir userId olan kullanıcılar için çalışır. Kararlı bir userId olmayan anonim doğrulanmamış yorumlar bunu tetiklemez.
- Tetikleyici yorum onaylandığında/görünür olduğunda tetiklenir (ilk gönderi anında değil). İlk yorumlarda yapılan düzenlemeler veya moderatör işlemleri tetikleyiciyi yeniden çalıştırmaz.

### Yaygın kullanımlar

- **Karşılama mesajı** - bu tetikleyici etrafında oluşturulmuş [Welcome Greeter şablonu](#template-welcome-greeter).
- **Onboarding** - kullanıcıya topluluk kurallarını işaret eden bir [DM uyarısı](#tool-warn-user) gönderin (burada ceza yerine dostça bir bilgilendirme olarak kullanılmıştır).
- **İnceleyici bildirimi** - her yeni yorumcunun ilk gönderisine bir insanın bakmasını istiyorsanız, [`mark_comment_reviewed`](#tools-overview) onları inceleme için işaretleyebilir.