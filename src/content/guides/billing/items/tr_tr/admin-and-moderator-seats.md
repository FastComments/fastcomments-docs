Flex'te, hesabınızı yönetebilen veya moderasyon yapabilen kişiler aylık koltuk (seat) olarak faturalandırılır. Bu, Flex faturasında insanları en çok şaşırtan kısımdır; genellikle koltukların kimsenin fark etmediği bir şekilde oluşturulmuş olmasından kaynaklanır.

### Bir Koltuk Sayılanlar

- **Yöneticiler (Admins)**: hesabınızın `Users` sayfasında herhangi bir yönetimsel izne (faturalandırma, özelleştirme, analitik, moderasyon vb.) sahip kullanıcılar. İlk yönetici, normalde hesap sahibi, ücretsizdir; her ek yönetici bir koltuktur.
- **Moderatörler**: `Moderators` sayfasında davetlerini kabul etmiş kişiler. Bekleyen bir davet faturalandırılmaz. Tek izni yorum moderasyonu olan bir kullanıcı, yönetici değil moderatör olarak faturalandırılır.
- **SSO Yöneticileri** ve **SSO Moderatörleri**: sitenizin SSO payload'ında `isAccountOwner`, `isAdminAdmin` veya `isCommentModeratorAdmin` ayarlanmış şekilde oturum açtırdığı SSO kullanıcıları. Bunlar normal yöneticiler ve moderatörlerle aynı oranlarda faturalandırılır. Bkz. [SSO User Structure](/guide-api.html#sso-user-structure).

Koltuklar e-posta adresine göre tekilleştirilir, yani hem yönetici hem moderatör olan biri veya aynı e-postayla hem SSO kullanıcısı hem panel girişi olan biri, daha yüksek oran üzerinden bir kez faturalandırılır. İki farklı e-posta adresine sahip biri iki koltuktur.

Aylık aktif kullanıcıya göre faturalandırılan hesaplarda, o ay içinde oturum açmamış bir SSO moderatörü o ay için faturalandırılmaz.

### Eklemediğiniz Koltuklar

İki özellik otomatik olarak koltuk oluşturur:

- **SSO yönetici senkronizasyonu.** Bir SSO kullanıcısı, panel kullanıcılarınızdan biriyle eşleşen bir e-posta ile oturum açtığında, o kullanıcının yönetici veya moderatör izinleri SSO kullanıcısına kopyalanır. WordPress SSO'da bu, her WordPress yöneticisinin FastComments yöneticisi veya moderatörü olabileceği anlamına gelir. `Customize` altındaki özelleştirme kuralınızda `Disable Automatic Admin Migration` seçeneğiyle bunu kapatın.
- **Aynı kişi için farklı e-postalar.** Panel girişiniz bir e-posta, SSO kullanıcınız başka bir e-posta kullanıyorsa, bunlar iki koltuktur. Her ikisinde de aynı e-postayı kullanmak bunları birleştirir.

### Koltukları Kontrol Etme ve Kaldırma

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) sayfası `Moderators`, `SSO Admins` ve `SSO Moderators` sayılarınızı listeler ve sonraki fatura önizlemesi bunların maliyetini gösterir. İhtiyacınız olmayan kişileri `Users` ve `Moderators` sayfalarından kaldırın veya SSO payload'ınızdan yönetici bayraklarını kaldırın. Koltuklar, ay sonunda fatura oluşturulduğunda sayılır, bu nedenle o tarihten önce kaldırılan bir koltuk o ay için faturalandırılmaz.

Her ikisini yönetme konusunda bkz. [Adding Moderators](/guide-users.html#adding-moderators) ve [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions).