FastComments SSO (<a href="#sso">ayrıntılar burada</a>), kullanıcılarınızın başka bir platforma giriş yapmak zorunda kalmadan yorum yapabilmesini sağlar.

Ancak bu tek başına yorum dizilerinizi güvence altına almaz, çünkü varsayılan olarak yorum verileri herkese açık bilgilerdir - sayfayı görüntüleyebilen herkes yorumları görebilir.

Bir ayarı değiştirerek, yorumların yalnızca bir yönetici veya geçerli bir SSO kullanıcısı tarafından alınabilmesi dışında getirilmesini kısıtlayabiliriz.

#### Kod Yazmadan Kurulum

SSO kurulduğunda, bir <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">özelleştirme kuralı</a> oluşturarak yorum dizilerimizi görüntülemeyi ve etkileşimde bulunmayı engelleyebiliriz.

Bunu yaparken SSO araması yapın ve bu seçeneği bulacaksınız:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Özelleştirme kuralında etkinleştirilmiş, güvenlik düzeyi seçimiyle birlikte Yorumları görüntülemek için SSO gerekli seçeneği'; title='Yorumları Görüntülemek İçin SSO Gerekli' app-screenshot-end]

Etkinleştirin ve özelleştirme kuralını kaydedin.

#### Yalnızca Belirli Bir Alan Adını veya Sayfayı Koruma

Yalnızca belirli bir Alan Adını veya Sayfayı korumak için, özelleştirme kuralını buna göre yapılandıracağız.

Özelleştirme arayüzünün üst kısmında, Alan Adı (Domain) ve URL Kimliği (URL ID) olmak üzere iki giriş alanı bulacağız.

Yalnızca belirli bir alan adını korumak için, ilgili alan adını "domain" alanına girin.

Belirli bir sayfayı korumak için, "URL ID" alanına bir sayfa URL'si girin. FastComments ile özel bir entegrasyonunuz varsa, URL yerine bir tür kimlik girebilirsiniz.

#### Güvenlik Düzeyleri

SSO zorunlu kılarken, Basit SSO mu yoksa Güvenli SSO mu gerektirdiğinize karar vermek isteyeceksiniz. Basit SSO gerektiriyorsanız ikisine de izin verilir, ancak Güvenli SSO gerektiriyorsanız içeriğin görüntülenebilmesi için API anahtarınızla hash'lenmiş bir Güvenli SSO payload'ı ile alınması gerekir.

Güvenlik düzeyi seçeneği, "Yorumları Görüntülemek İçin SSO Gerekli" seçeneğini seçtiğinizde görünür ve yalnızca bu seçenek açıkken geçerlidir.

<a href="#sso-simple">Yorumları Basit SSO ile Otomatik Doğrula ve Basit SSO Yöneticilerine İzin Ver</a> ayarları ayrıdır ve bu seçeneğin açık olup olmamasından bağımsız olarak uygulanır.

#### Okumanın Ötesinde Koruma

Bu seçeneği etkinleştirmek, kullanıcı SSO ile giriş yapmadığı sürece sayfanın veya alan adının yorum yapılmaktan korunmasını sağlar.

#### Dikkat Edilmesi Gerekenler

SSO entegrasyonunuzdan önce yorum oluşturan kullanıcılar, SSO entegrasyonunuz üzerinden giriş yapmadıkları sürece yorumlarını göremezler.