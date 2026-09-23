[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Bir sayfadaki beğeni sayısını ve mevcut kullanıcının sayfayı beğenip beğenmediğini döndürür. Henüz var olmayan sayfalar `likeCount` değeri `0` olarak döner.

[inline-code-attrs-start title = 'Sayfa Beğenileri cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Beğenileri İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** SSO nesnenizin URI kodlu JSON'u. Anonim kullanıcılar için atlayın. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Beğenileri Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: string
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    likeCount: number
    /** İsteği yapan kullanıcının sayfayı beğenip beğenmediği. **/
    didLike: boolean
    /** Sayfadaki üst düzey yorumların sayısı. **/
    commentCount: number
    /** Bu sayfa için canlı güncellemelere abone olmak için kullanılan kimlik. **/
    urlIdWS: string
}
[inline-code-end]