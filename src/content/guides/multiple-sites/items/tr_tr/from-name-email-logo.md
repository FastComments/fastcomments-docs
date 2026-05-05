Bazen FastComments kullanıcılarınıza e-posta göndermek zorunda kalır, özellikle Secure SSO kullanmıyorsanız.

Bunun örnekleri, ilk kez yorum yaparken hesaplarını veya etkinliklerini doğrulamak içerir. FastComments ayrıca yorumlarına yapılan yanıtlar için bildirimler de gönderir.

FastComments kullanıcılarınıza e-posta gönderdiğinde, varsayılan Gönderen Adı ve E-posta olarak `FastComments Robot` ve `noreply@fastcomments.com` kullanırız.

Bu e-postaların altbilgisinde kendi logomuzu da kullanırız.

Eğer FastComments Flex veya Pro'ya sahipseniz, bunların tümü alan-bağımlı olarak "My Domains page" üzerinden özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

E-postalarda gösterilen logoyu özelleştirirken, yüklediğiniz boyutun e-postanın altbilgisinde göstermek istediğiniz boyutla aynı olduğundan emin olun.

### When Customizing The `From Domain`

Eğer `From Domain`'ı özelleştirirseniz, e-posta sağlayıcıları ve istemciler FastComments'ın adınıza e-posta göndermeye yetkili olduğunu bilmelidir. Aksi takdirde,
`From Domain`'ı tanımlayıp aşağıdaki adımları takip etmemek, e-postaların spam klasörüne gitmesine yol açabilir.

#### 1. Setup SPF

FastComments'ın alan adınız adına güvenli bir şekilde e-posta göndermesine izin vermek için bize izin veren bir SPF kaydı eklediğinizden emin olun.

Alan adınız adına `mail.fastcomments.com` ve `sib.fastcomments.com`'un posta göndermesine izin veren SPF kayıtlarının olduğundan emin olun.

Bunu nasıl yapacağınıza dair daha fazla bilgi burada: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

SPF'ye ek olarak DKIM'i de yapılandırmalısınız. DNS yapılandırmanız hazır olduğunda, alan yapılandırmaları sayfasında "Show Advanced" seçeneğine tıklayarak alan başına DKIM ayarlarını gösterebilirsiniz.

DKIM yapılandırmasını ayarlamak için [API'yi çağırabilirsiniz](/guide-api.html#domain-config-structure).

### Unsubscribe Links

SSO kullanıldığında, e-posta ve bildirimlerde kullanılan abonelikten çıkma özellikleri [DomainConfigs API](/guide-api.html#domain-config-structure) aracılığıyla özelleştirilebilir.

### Email Link Obfuscation

Eğer sitenizin alan adı itibarı bildirim e-postalarının spam klasörüne düşmesine neden oluyorsa, "view comment" düğmelerini doğrudan sayfanıza bağlamak yerine `fastcomments.com` üzerinden yönlendirebilirsiniz. Posta sağlayıcıları e-posta gövdesindeki her bağlantıyı hedefin itibarına göre puanlar; bu yüzden alan adınız işaretleniyorsa çıplak bağlantılar, gönderim ayarlarınız ne kadar temiz olursa olsun spam puanına katkıda bulunur.

Bunu, My Domains sayfasında "Show Advanced" altında, "Email Link Obfuscation" bölümünde etkinleştirin. Ayar alan başına geçerlidir.

Etkinleştirildiğinde, links in mention, reply, new-comment, subscribed-page, profile-comment, and digest e-postalarındaki bağlantılar, tıklanınca orijinal sayfaya yönlendiren kısa tokenlara yeniden yazılır. Hedef kiracıınıza bağlıdır: yönlendirme yalnızca hostu yapılandırılmış alanlarınızdan biriyle eşleşen URL'lere iletir ve tokenlar 30 gün sonra otomatik olarak süresi dolar.

Tıklanan bağlantıdaki deneyim değişmez. Okuyucular yine yorumun görünür olması için sayfanıza yönlendirilir.