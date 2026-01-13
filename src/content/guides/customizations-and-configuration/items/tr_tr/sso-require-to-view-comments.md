FastComments SSO (<a href="#sso">ayrıntılar burada</a>) kullanıcılarınıza başka bir platforma giriş yapmadan yorum yapma imkanı sağlar.

Ancak bu tek başına yorum dizilerinizi güvence altına almaz, çünkü varsayılan olarak yorum verileri herkese açık bilgidir - sayfayı görüntüleyebilen herkes yorumları görebilir.

Bir ayarı değiştirerek, yorumların yalnızca bir yönetici veya geçerli bir SSO kullanıcısı tarafından alınmasına izin verecek şekilde sınırlandırabiliriz.

#### Kod Gerektirmeyen Kurulum

SSO kurulduğunda yorum dizilerimizin görüntülenmesini ve etkileşimde bulunulmasını engellemek için bir <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">özelleştirme kuralı</a> oluşturabiliriz.

Bunu yaparken SSO'yu arayın ve şu seçeneği bulacaksınız:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Etkinleştirin ve özelleştirme kuralını kaydedin.

#### Yalnızca Belirli Bir Alan Adı veya Sayfayı Korumak

Yalnızca belirli bir Domain veya Sayfayı korumak için özelleştirme kuralını buna göre yapılandırmamız yeterlidir.

Özelleştirme arayüzünün üst kısmında iki girdi alanı bulacaksınız: Domain ve URL ID.

Sadece belirli bir domaini korumak için ilgili domaini "domain" alanına girin.

Belirli bir sayfayı korumak için sayfa URL'sini "URL ID" alanına girin. FastComments ile özel bir entegrasyonunuz varsa, burada bir URL yerine bir tür ID girebilirsiniz.

#### Güvenlik Düzeyleri

SSO gerekli olduğunda Simple SSO mu yoksa Secure SSO mu istediğinize karar vermelisiniz. Simple SSO'yu seçerseniz her iki yöntem de kabul edilir; ancak Secure SSO'yu seçerseniz içeriğin görüntülenebilmesi için API key'inizle hash'lenmiş bir Secure SSO yükü ile getirilmesi gerekir.

Güvenlik düzeyi seçeneği, "Require SSO To View Comments" seçeneğini seçtiğinizde görünecektir.

#### Okumanın Ötesinde Koruma

Bu seçeneği etkinleştirmek, kullanıcı SSO ile giriş yapmadıkça sayfanın veya domainin yorum yapılmasına karşı korunmasını sağlar.

#### Uyarılar

SSO entegrasyonunuzdan önce yorum oluşturan kullanıcılar, SSO entegrasyonunuz aracılığıyla giriş yapmadıkça bunları göremeyeceklerdir.