FastComments bazen kullanıcılarınıza e-posta göndermek zorunda kalır, özellikle Secure SSO kullanmıyorsanız.

Buna örnek olarak, ilk kez yorum yaptıklarında hesaplarını veya etkinliklerini doğrulamak verilebilir. FastComments
ayrıca yorumlarına gelen yanıtlar için onlara bildirimler gönderir.

FastComments kullanıcılarınıza e-posta gönderdiğinde, varsayılan olarak `FastComments Robot` Gönderen Adı ve `noreply@fastcomments.com` Gönderen E-postası kullanılacaktır.

Bu e-postaların alt bilgisinde (footer) ayrıca kendi logomuzu kullanırız.

FastComments Flex veya Pro aboneliğiniz varsa, tüm bunlar "My Domains sayfası" üzerinden alan adı (domain) bazında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Gönderen Adı, Gönderen E-postası ve logo yükleme alanlarını içeren alan adı bazlı e-posta ayarları formu'; title='Gönderen Adı, E-posta ve Logoyu Özelleştirme' app-screenshot-end]

E-postalarda gösterilen logoyu özelleştirirken, yüklediğiniz boyutun e-postanın alt bilgisinde gösterilmesini istediğiniz boyutla aynı olduğundan emin olun.

### `From Domain` Özelleştirilirken

`From Domain` değerini özelleştirirseniz, e-posta sağlayıcıları ve istemcileri FastComments'un sizin adınıza e-posta göndermeye yetkili olduğunu bilmelidir. Aksi takdirde,
`From Domain` tanımlayıp aşağıdaki adımları izlememek, büyük olasılıkla e-postaların spam'e gitmesine neden olur.

#### 1. SPF Kurulumu

FastComments'un alan adınız adına güvenli bir şekilde e-posta gönderebilmesine izin vermek için, bunu mümkün kılan bir SPF kaydı eklediğinizden emin olun.

`mail.fastcomments.com` ve `sib.fastcomments.com` adreslerinin alan adınız adına e-posta gönderebilmesine izin veren SPF kayıtlarının bulunduğundan emin olun.

Bunun nasıl yapılacağı hakkında daha fazla bilgi burada: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM Kurulumu

SPF'ye ek olarak, DKIM kurulumunu da yapmalısınız. DNS yapılandırmanız hazır olduğunda, alan adı yapılandırma sayfasında "Show Advanced" seçeneğine tıklayarak
alan adı bazında DKIM ayarlarını görüntüleyebilirsiniz.

Ayrıca DKIM yapılandırmasını ayarlamak için [API'yi çağırabilirsiniz](/guide-api.html#domain-config-structure).

### Abonelikten Çıkma Bağlantıları

SSO kullanırken, e-postalarda ve bildirimlerde kullanılan abonelikten çıkma özellikleri [DomainConfigs API'si aracılığıyla](/guide-api.html#domain-config-structure) özelleştirilebilir.

### E-posta Bağlantısı Gizleme (Obfuscation)

Sitenizin alan adı itibarı, bildirim e-postalarının spam'e düşmesine neden oluyorsa, "yorumu görüntüle" düğmelerini doğrudan sayfanıza bağlamak yerine `fastcomments.com` üzerinden yönlendirebilirsiniz. E-posta sağlayıcıları, e-posta gövdesindeki her bağlantıyı hedefin itibarına göre puanlar; bu nedenle alan adınız işaretlenmişken çıplak bağlantılar, gönderim kurulumunuz ne kadar temiz olursa olsun spam puanına katkıda bulunur.

Bunu My Domains sayfasındaki "Show Advanced" bölümünde, "Email Link Obfuscation" bölümünde etkinleştirebilirsiniz. Bu ayar alan adı bazındadır.

Etkinleştirildiğinde, bahsetme (mention), yanıt, yeni yorum, abone olunan sayfa, profil yorumu ve özet e-postalarındaki bağlantılar, tıklandığında orijinal sayfaya yönlendiren kısa token'lara dönüştürülür. Hedef, kiracınıza (tenant) bağlıdır: yönlendirme yalnızca ana makinesi (host) yapılandırılmış alan adlarınızdan biriyle eşleşen URL'lere iletilir ve token'lar 30 gün sonra otomatik olarak sona erer.

Tıklama sonrası deneyim değişmez. Okuyucular yine sayfanıza, yorum görünüme kaydırılmış şekilde ulaşır.