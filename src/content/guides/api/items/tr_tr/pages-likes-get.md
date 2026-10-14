[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Bir sayfadaki beğeni sayısını ve isteğe bağlı olarak beğenen kullanıcıları, en yeni önce döndürür. Bu, [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get) API'sinin sunucu tarafı sürümüdür.

`includeUserInfo` hangi kullanıcıların döndürüleceğini kontrol eder:

- `public` profil etkinliğini gizli yapan kullanıcıları dışarı bırakır.
- `include-private` onları dahil eder. Bu seçeneği yalnızca liste halka gösterilmeyecekse kullanın.

Anonim kullanıcılar hiçbir zaman dahil edilmez ve en fazla 100 beğeni sorgulanır. Var olmayan sayfalar `likeCount` değeri `0` ve boş bir `users` listesi döndürür.

[inline-code-attrs-start title = 'Sayfa Beğenileri cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Beğenileri İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Beğenileri Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    likeCount?: number
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Faydalı İpucu

`urlId` gibi değerleri URI Encode etmeyi unutmayın.