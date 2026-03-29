### FastComments API

FastComments, birçok kaynakla etkileşim kurmak için bir API sağlar. Platformumuzla entegrasyonlar oluşturun veya kendi istemcilerinizi bile yazın!

Bu dokümantasyonda, API tarafından desteklenen tüm kaynakları istek ve yanıt tipleriyle birlikte bulacaksınız.

Kurumsal müşteriler için tüm API erişimleri Denetim Günlüğünde kaydedilir.

### Oluşturulan SDK'lar

FastComments artık kodumuzdan bir [API Şeması](https://fastcomments.com/js/swagger.json) oluşturuyor (bu henüz tamamlanmadı, ancak birçok API'yi içeriyor).

Ayrıca popüler diller için artık SDK'larımız var:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Kimlik Doğrulama

API, [api anahtarınızı](https://fastcomments.com/auth/my-account/api-secret) ya `X-API-KEY` başlığı olarak ya da `API_KEY` sorgu parametresi olarak göndererek doğrular. Ayrıca API çağrıları yapmak için `tenantId`'ye de ihtiyacınız olacak. Bu, api anahtarınızla aynı sayfadan alınabilir.

### Güvenlik Notu

Bu yollar **bir sunucudan** çağrılmak içindir. __ASLA__ bunları bir tarayıcıdan çağırmayın. Bunu yapmak API anahtarınızı açığa çıkarır - bu, bir sayfanın kaynak kodunu görebilen herhangi bir kişiye hesabınıza tam erişim sağlayacaktır!

#### Kimlik Doğrulama Seçeneği Bir - Başlıklar

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Kimlik Doğrulama Seçeneği İki - Sorgu Parametreleri

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Kendi Yazdıklarınızı Okuma

FastComments Active-Active kullanılabilirlik sağlar. Veri merkezinize yapılan istekler size en yakın [erişim noktasına](https://sophon.fastcomments.com/) yönlendirilir. Bu otomatikdir ve normalde yazdığınızı okuma (read-your-write) semantiğini gözlemleyebilirsiniz. Yazdıklarınızı kesin olarak okumak isterseniz, isteklerinizi belirli bir bölgeye o bölgeyi API hostu olarak kullanarak sabitleyebilirsiniz (ancak bu, çoğu entegrasyon için genellikle gerekli değildir):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Bunu yaparsanız bir yedek (fallback) tanımlamak isteyebilirsiniz; geçmişte giriş noktası düğümlerini kullanımdan kaldırdık ve geçiş için yeni adlar kullanıyoruz.