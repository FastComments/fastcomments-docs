FastComments bir OAuth 2.1 yetkilendirme sunucusudur. Bir uygulama, bir FastComments hesabına bağlı bir token alabilir ve bu kılavuzdaki her uç noktada API anahtarı yerine kullanabilir. Bu, Zapier uygulaması, MCP sunucusu ve diğer üçüncü taraf entegrasyonların bağlanma şeklidir.

Tokenlar, PKCE ile yetkilendirme kodu akışı üzerinden verilir. İstemci kimlik bilgileri veya örtük izin yoktur.

### Keşif

Uç nokta konumları, desteklenen izinler ve kimlik doğrulama yöntemleri standart metaveri URL'sinde yayınlanır:

[inline-code-attrs-start title = 'Yetkilendirme Sunucusu Metaverisi'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Açıklanan uç noktalar:

[inline-code-attrs-start title = 'OAuth Uç Noktaları'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

AB bölgesindeki hesaplar, aynı yollarla, `https://eu.fastcomments.com` adresini sağlayıcı olarak kullanır.

### Bir istemci kaydetme

Bir istemcinin akışı başlatmadan önce bir `client_id` ve kayıtlı bir `redirect_uri`'ye ihtiyacı vardır. Bunu elde etmenin iki yolu vardır:

- **Dinamik İstemci Kaydı.** RFC 7591'e göre bir JSON gövdesiyle `POST /oauth/register` (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Yanıt `client_id` ve gizli istemciler için `client_secret` içerir. Kayıt kimlik doğrulaması gerektirmez ve IP başına oran sınırlıdır.
- **İstemci ID Metaveri Belgesi.** İstemci, kontrol ettiği bir `https` URL'sini `client_id` olarak kullanır. FastComments bu URL'yi alır ve aynı metaveri alanlarını okur. Kayıt çağrısına gerek yoktur.

FastComments kontrol panelinde listelenen ortak uygulamalar, örneğin Zapier, doğrudan FastComments tarafından kaydedilir. Bir pazar yeri listesi oluşturuyorsanız ve bir birinci taraf istemciye ihtiyacınız varsa destek ile iletişime geçin.

### Kapsamlar

[inline-code-attrs-start title = 'Kapsamlar'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Hiçbir kapsam talep etmeyen bir istek her iki kapsamı da alır. Kullanıcı, istenen kapsamları onay sayfasında görür. Bu iki kapsam dışındaki bir kapsam talebi `invalid_scope` hatasıyla başarısız olur.

### Adım 1 - Yetkilendirme isteği

Kullanıcının tarayıcısını yetkilendirme uç noktasına yönlendirin. PKCE, `S256` yöntemiyle her istemci için gereklidir.

[inline-code-attrs-start title = 'Yetkilendirme İsteği'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = 'Yetkilendirme İsteği Parametreleri'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Must exactly match one of the client's registered redirect URIs. **/
    redirect_uri: string
    /** Space separated. Omit to request both scopes. **/
    scope?: 'read' | 'write' | 'read write'
    /** Returned unchanged on the redirect. Use it to bind the callback to the session that started the flow. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Optional RFC 8707 resource indicator. If sent, the same value must be sent to the token endpoint. **/
    resource?: string
}
[inline-code-end]

Kullanıcı gerektiğinde FastComments'a giriş yapar ve uygulamanızın adını, bağlanacağı hesabı ve istenen kapsamları gösteren bir onay sayfası görür. Kullanıcının o hesapta **API Admin** iznine sahip olması gerekir; başka birisi onay formu yerine izin hatası alır. Onaylamak, tarayıcıyı `code` ve `state` ile `redirect_uri`'nize yönlendirir. Reddetmek, `error=access_denied` ile yönlendirir.

Yetkilendirme kodu 10 dakika geçerlidir ve bir kez takas edilebilir. Aynı kodun ikinci takası, ilk takasın ürettiği tüm tokenları iptal eder.

### Adım 2 - Token isteği

Kodu tokenlar için takas edin. Gövde form-encoded'dir. Gizli istemciler `client_secret_basic` (HTTP Basic) veya `client_secret_post` (gövdede gizli) ile kimlik doğrular. Genel istemciler sadece `client_id` gönderir.

[inline-code-attrs-start title = 'Token İsteği cURL Örneği'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = 'Token İsteği Gövdesi (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Confidential clients only. May be sent as HTTP Basic auth instead. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Must match the authorization request when sent. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Token Yanıtı Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Prefixed fcat_. Valid for one hour. **/
    access_token: string
    token_type: 'bearer'
    /** Seconds until the access token expires. 3600. **/
    expires_in: number
    /** Prefixed fcrt_. Valid for 30 days from issue. **/
    refresh_token: string
    /** Space separated scopes granted. **/
    scope: string
}
[inline-code-end]

Hatalar RFC 6749'e uyar: `error` ve `error_description` içeren bir JSON gövdesi, `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` ve `unsupported_grant_type` için HTTP 400, `invalid_client` için HTTP 401, oran sınırlaması olduğunda HTTP 429.

### Adım 3 - API'yi Çağırma

Erişim tokenını bir bearer token olarak gönderin. Kiracı token tarafından ima edilir, bu yüzden `tenantId` isteğe bağlıdır. Verildiğinde tokenla eşleşmelidir, aksi takdirde istek başarısız olur.

[inline-code-attrs-start title = 'Bearer Token cURL Örneği'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` kiracıyı, yetkilendiren kullanıcıyı ve verilen kapsamları döndürür; bu, bağlantı testi için doğru çağrıdır. Süresi dolmuş veya iptal edilmiş bir tokenla yapılan istek HTTP 401 alır. Yöntemi, tokenın sahip olmadığı bir kapsam gerektiren bir istek HTTP 403 alır.

### Adım 4 - Yenileme

[inline-code-attrs-start title = 'Yenileme İsteği cURL Örneği'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Token İsteği Gövdesi (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Optional. Narrows to a subset of the scopes originally granted. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Yanıt, kod takasıyla aynı yapıya sahiptir. Yenileme tokenları döner: her yenileme yeni bir `refresh_token` döndürür ve eşzamanlı istekler için 30 saniyelik bir gecikme penceresinden sonra eski tokenı iptal eder. 30 saniyeden daha uzun bir sürede dönen bir yenileme tokenı sunulursa bu tekrar olarak değerlendirilir ve tüm izin iptal edilir. FastComments tarafından kaydedilen ortak uygulamalar döndürmeden muaf tutulur ve aynı yenileme tokenını alır; süresi başka 30 gün uzatılır.

Yenileme ayrıca yetkilendiren kullanıcının hâlâ hesapta API Admin iznine sahip olup olmadığını kontrol eder. Eğer yoksa, izin iptal edilir ve yanıt `invalid_grant` olur.

### İptal

[inline-code-attrs-start title = 'İptal İsteği cURL Örneği'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Bir yenileme tokenını iptal etmek, aynı izinden verilen tüm erişim tokenlarını iptal eder. Bir erişim tokenını iptal etmek sadece o tokenı iptal eder. Uç nokta, token bulunup bulunmadığına bakılmaksızın boş bir JSON nesnesiyle HTTP 200 döner, RFC 7009'e göre.

Kullanıcılar ayrıca FastComments kontrol panelindeki **Bağlı Uygulamalar** üzerinden bir bağlantıyı iptal edebilir. O uygulama için tüm tokenlar hemen çalışmaz.