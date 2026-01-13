### FastComments API

FastComments, birçok kaynakla etkileşim kurmak için bir API sağlar. Platformumuzla entegrasyonlar oluşturun veya kendi istemcilerinizi dahi yazın!

Bu dokümantasyonda, API tarafından desteklenen tüm kaynakları ve bunların istek ve yanıt tiplerini bulacaksınız.

Kurumsal (Enterprise) müşteriler için, tüm API erişimleri Denetim Günlüğü'ne (Audit Log) kaydedilir.

### Oluşturulan SDK'lar

FastComments artık kodumuzdan bir [API Spec](https://fastcomments.com/js/swagger.json) üretiyor (bu henüz tamamlanmadı, ancak birçok API'yi içeriyor).

Ayrıca popüler diller için SDK'larımız da mevcut:

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

API, [api key](https://fastcomments.com/auth/my-account/api-secret) değerini ya `X-API-KEY` başlığı olarak ya da `API_KEY` sorgu parametresi olarak ileyerek kimlik doğrulaması yapar. API çağrıları yapmak için ayrıca `tenantId` değerine de ihtiyacınız olacaktır. Bu değer, api key ile aynı sayfadan alınabilir.

### Güvenlik Notu

Bu yolların bir **sunucudan** çağrılması amaçlanmıştır. __YAPMAYIN__ bunları bir tarayıcıdan çağırmayın. Bunu yapmak API key'inizi açığa çıkarır - bir sayfanın kaynak kodunu görebilen herkese hesabınıza tam erişim sağlar!

#### Kimlik Doğrulama Seçeneği Bir - Başlıklar

- Başlık: `X-API-KEY`
- Başlık: `X-TENANT-ID`

#### Kimlik Doğrulama Seçeneği İki - Sorgu Parametreleri

- Sorgu Parametresi: `API_KEY`
- Sorgu Parametresi: `tenantId`

---