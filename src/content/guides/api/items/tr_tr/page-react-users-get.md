[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Sayfaya bir reaksiyon ekleyen kullanıcıların adlarını alfabetik olarak sıralar. En fazla 100 reaksiyon aranır ve anonim kullanıcılar dahil edilmez.

[inline-code-attrs-start title = 'Sayfa Reaksiyon Kullanıcıları cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Reaksiyon Kullanıcıları İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** Reaksiyon kimliği. **/
    id: string
    /** SSO nesnenizin URI kodlu JSON'u. Anonim kullanıcılar için atlayın. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Reaksiyon Kullanıcıları Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: string
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]