---
FastComments SSO (<a href="#sso">detaylar burada</a>) kullanıcılarınıza başka bir platforma giriş yapmadan yorum yapma imkanı sunar.

Ancak, bu tek başına yorum dizilerinizi güvenli hale getirmez, çünkü varsayılan olarak yorum verileri herkese açık bir bilgi olarak sunulur - sayfayı görebilen herkes yorumları görebilir.

Bir ayarı değiştirerek, yorumların yalnızca bir yönetici veya geçerli bir SSO kullanıcısı tarafından alınmasını kısıtlayabiliriz.

#### Kod Olmadan Kurulum

SSO kurulduğunda, yorum dizilerimizi görüntülemeyi ve etkileşime girmeyi, bir <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">özelleştirme kuralı</a> oluşturarak önleyebiliriz.

Bunu yaparken, SSO'yu arayın ve bu seçeneği bulacaksınız:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Özelleştirme kuralında yorumları görüntülemek için SSO gerektirme seçeneği, güvenlik seviyesi seçeneğiyle etkinleştirildi'; title='Yorumları Görüntülemek İçin SSO Gerektir' app-screenshot-end]

Bunu etkinleştirin ve özelleştirme kuralını kaydedin.

#### Yalnızca Belirli Bir Alan Adı veya Sayfayı Korumak

Yalnızca belirli bir alan adı veya sayfayı korumak için, özelleştirme kuralını buna göre yapılandıracağız.

Özelleştirme arayüzünün üst kısmında iki giriş alanı bulacağız: Domain ve URL ID.

Sadece belirli bir alan adını korumak için, ilgili alan adını "domain" alanına girin.

Belirli bir sayfayı korumak için, "URL ID" alanına bir sayfa URL'si girin. FastComments ile özel bir entegrasyonunuz varsa, burada URL yerine bir kimlik türü girebilirsiniz.

#### Güvenlik Seviyeleri

SSO gerektirdiğinizde, Basit SSO mu yoksa Güvenli SSO mu istediğinize karar vermeniz gerekir. Basit SSO'yu seçerseniz, her ikisi de izin verilir, ancak Güvenli SSO'yu seçerseniz, içeriğin görüntülenebilmesi için API anahtarınızla hashlenmiş bir Güvenli SSO yüküyle alınması gerekir.

Güvenlik seviyesi seçeneği, "Yorumları Görüntülemek İçin SSO Gerektir" seçeneğini seçtiğinizde görünecektir.

#### Okumanın Ötesinde Koruma

Bu seçeneği etkinleştirmek, kullanıcı SSO üzerinden oturum açmadıkça sayfanın veya alan adının yorumlanmasını engelleyecektir.

#### Dikkat Edilmesi Gerekenler

SSO entegrasyonunuzdan önce yorum oluşturan kullanıcılar, SSO entegrasyonunuz üzerinden oturum açmadıkça bu yorumları göremezler.