### FastComments API'si

FastComments, birçok kaynakla etkileşim kurmak için bir API sağlar. Platformumuzla entegrasyonlar oluşturabilir ya da hatta kendi istemcilerinizi geliştirebilirsiniz!

Bu belgede, API tarafından desteklenen tüm kaynakları, istek ve yanıt türleriyle birlikte bulacaksınız.

Kurumsal müşteriler için, tüm API erişimleri Denetim Günlüğü'nde kaydedilir.

### Oluşturulan SDK'lar

FastComments artık kodumuzdan bir [API Spec](https://fastcomments.com/js/swagger.json) oluşturuyor (bu henüz tam değil, ancak birçok API'yi içeriyor).

Ayrıca popüler diller için SDK'larımız da var:

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

API, [api anahtarınızı](https://fastcomments.com/auth/my-account/api-secret) `X-API-KEY` başlığı ya da `API_KEY` sorgu parametresi olarak göndererek kimlik doğrulaması yapar. API çağrıları yapmak için ayrıca `tenantId`'ye de ihtiyacınız olacak. Bu, api anahtarınızla aynı sayfadan alınabilir.

### Güvenlik Notu

Bu yollar **sunucudan** çağrılmak üzere tasarlanmıştır. __ÇAĞIRMAYIN__ bir tarayıcıdan. Bunu yaparsanız API anahtarınız ortaya çıkar - bu, sayfanın kaynak kodunu görebilen herkese hesabınıza tam erişim sağlar!

#### Kimlik Doğrulama Seçeneği Bir - Başlıklar

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Kimlik Doğrulama Seçeneği İki - Sorgu Parametreleri

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Kimlik Doğrulama Seçeneği Üç - OAuth Bearer Token

- Header: `Authorization: Bearer fcat_...`

Zapier gibi üçüncü taraf uygulamalar ve [MCP sunucusunun](https://docs.fastcomments.com/guide-llm-kit.html) istemcileri, API anahtarı yerine OAuth aracılığıyla bir token alır. Bu token burada her uç noktada çalışır. Tenant, token tarafından ima edilir, bu yüzden `tenantId` isteğe bağlıdır, ancak verildiğinde token ile eşleşmelidir. `GET` istekleri `read` kapsamına, diğer tüm yöntemler `write` kapsamına ihtiyaç duyar. İstemci kaydı, PKCE, yenileme ve iptal dahil tam akış [OAuth Yetkilendirme](#oauth) altında belgelenmiştir. Keşif `https://fastcomments.com/.well-known/oauth-authorization-server` adresinde başlar.

### Kendi Yazılarınızı Okuma

FastComments, Active-Active kullanılabilirlik sağlar. Veri merkezinizden gelen istekler, size en yakın [varlık noktasına](https://sophon.fastcomments.com/) yönlendirilir. Bu otomatik bir işlemdir ve genellikle okuma-yazma tutarlılığını gözlemleyebilirsiniz. Kendi yazılarınızı okuduğunuzdan emin olmak istiyorsanız, isteklerinizi belirli bir bölgeye sabitleyebilir ve o bölgeyi API ana bilgisayarı olarak kullanabilirsiniz (ancak çoğu entegrasyon için genellikle gerekmez):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Bunu yaparsanız, geçmişte giriş noktası düğümlerini kaldırdığımız ve geçiş için yeni adlar kullandığımız için bir yedek tanımlamak isteyebilirsiniz.