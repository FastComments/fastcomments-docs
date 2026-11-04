FastComments SSO (<a href="#sso">buradaki detaylar</a>) kullanıcılarınıza başka bir platforma giriş yapmadan yorum yapma imkanı sağlar.

Ancak, bu tek başına yorum dizilerinizi güvence altına almaz, çünkü varsayılan olarak yorum verileri herkese açık bir bilgi olarak bulunur - sayfayı görebilen herkes yorumları görebilir.

Bir ayarı değiştirerek, yorumların yalnızca bir yönetici veya geçerli bir SSO kullanıcısı tarafından alınmasını kısıtlayabiliriz.

#### No-Code Setup

SSO kurulduğunda, yorum dizilerimizi görüntülemeyi ve etkileşimi önlemek için bir <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">özelleştirme kuralı</a> oluşturabiliriz.

Bunu yaparken, SSO'yu arayın ve bu seçeneği bulacaksınız:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Özelleştirme kuralında etkinleştirilen, yorumları görüntülemek için SSO gerektirme seçeneği, güvenlik seviyesi seçeneğiyle'; title='Yorumları Görüntülemek İçin SSO Gerektir' app-screenshot-end]

Bunu etkinleştirin ve özelleştirme kuralını kaydedin.

#### Only Protect a Certain Domain or Page

Belirli bir alan adı veya sayfayı yalnızca korumak için, özelleştirme kuralını buna göre yapılandıracağız.

Özelleştirme arayüzünün üst kısmında, Domain ve URL ID olmak üzere iki girdi bulacağız.

Sadece belirli bir alan adını korumak için, ilgili alan adını "domain" alanına girin.

Belirli bir sayfayı korumak için, "URL ID" alanına bir sayfa URL'si girin. FastComments ile özel bir entegrasyonunuz varsa, burada URL yerine bir kimlik türü girebilirsiniz.

#### Security Levels

SSO gerektirirken, Simple SSO mu yoksa Secure SSO mu istediğinize karar vermeniz gerekir. Simple SSO'yu seçerseniz, her ikisi de izinlidir, ancak Secure SSO'yu seçerseniz, içeriğin görüntülenebilmesi için API anahtarınızla hashlenmiş bir Secure SSO yüküyle alınması gerekir.

"Yorumları Görüntülemek İçin SSO Gerektir" seçeneğini seçtiğinizde güvenlik seviyesi seçeneği görünecek ve yalnızca bu seçenek açıkken etkili olacaktır.

<a href="#sso-simple">Simple SSO ile Yorumları Otomatik Doğrulama ve Simple SSO Yöneticilerine İzin Ver</a> ayarları ayrı olup, bu seçeneğin açık olup olmamasına bakılmaksızın uygulanır.

#### Protection Beyond Reading

Bu seçeneği etkinleştirmek, kullanıcı SSO üzerinden oturum açmadıkça sayfanın veya alan adının yorumlanmasını engelleyecektir.

#### Gotchas

SSO entegrasyonunuzdan önce yorum oluşturan kullanıcılar, SSO entegrasyonunuz üzerinden oturum açmadıkça bu yorumları göremezler.