[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Bir sayfaya tepki ekleyen kullanıcıların adlarını alfabetik olarak döndürür. En fazla 100 tepki aranır ve anonim kullanıcılar ile özel profil etkinliği olan kullanıcılar dahil edilmez. SSO gerektiren sayfalarda, sayfayı görüntüleyemeyen kullanıcılara `sso-required` veya `page-forbidden` kodlu bir `403` hatası döner.

Avatarları ve kullanıcı kimliklerini de almak için `includeUserInfo=public` parametresiyle [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) isteğini kullanın.

[inline-code-attrs-start title = 'Sayfa Tepki Kullanıcıları cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Tepki Kullanıcıları İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Tepki Kullanıcıları Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]