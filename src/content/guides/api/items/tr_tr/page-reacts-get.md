[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Bir sayfadaki her tepkinin sayısını ve mevcut kullanıcının eklediği tepkileri döndürür.

`includeUserInfo=public` parametresini göndererek, tepki veren kullanıcıları da alabilirsiniz; her tepki için tepki kimliğiyle bir giriş, en yenisi ilk. Özel profil etkinliği olan kullanıcılar ve anonim kullanıcılar dışarıda bırakılır ve en fazla 100 tepki aranır. Burada yalnızca `public` kabul edilir; `include-private` bir `422` hatası döndürür ve bir API anahtarıyla [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) üzerinden kullanılabilir. SSO gerektiren sayfalarda, `includeUserInfo` sayfayı görüntüleyemeyen kullanıcılar için bir `403` döndürür.

[inline-code-attrs-start title = 'Sayfa Tepkileri cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Tepkileri İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** SSO nesnenizin URI kodlu JSON'u. Anonim kullanıcılar için atlayın. **/
    sso?: string
    /** 'public' olarak ayarlayın, böylece tepki veren kullanıcıları dahil eder. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Tepkileri Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    /** Tepki kimliğine göre sayım, örneğin {"heart": 12, "laugh": 3}. Sayfa hiç tepki almadığında ayarlanmaz. **/
    counts?: Record<string, number>
    /** İsteği yapan kullanıcının eklediği tepki kimlikleri. Hiç eklemediyse ayarlanmaz. **/
    reactedIds?: string[]
    /** Her tepki için bir giriş, en yenisi ilk. Yalnızca includeUserInfo gönderildiğinde ayarlanır. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** Tepki kimliği, örneğin "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]