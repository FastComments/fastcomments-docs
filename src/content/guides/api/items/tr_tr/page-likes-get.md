[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Bir sayfadaki beğeni sayısını ve mevcut kullanıcının beğenip beğenmediğini döndürür. Henüz var olmayan sayfalar `likeCount` olarak `0` döndürür.

`includeUserInfo=public` parametresini göndererek sayfayı beğenen kullanıcıları da alabilirsiniz, en yeni önce. Özel profil etkinliği olan kullanıcılar ve anonim kullanıcılar dışarıda bırakılır ve en fazla 100 beğeni sorgulanır. Burada sadece `public` kabul edilir; `include-private` bir `422` döndürür ve bir API anahtarı ile [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) üzerinden kullanılabilir. SSO gerektiren sayfalarda, `includeUserInfo` sayfayı görüntüleyemeyen kullanıcılar için bir `403` döndürür.

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
    /** 'public' olarak ayarlayın, sayfayı beğenen kullanıcıları dahil etmek için. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Beğenileri Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    likeCount: number
    /** İsteği yapan kullanıcının sayfayı beğenip beğenmediği. **/
    didLike: boolean
    /** Sayfadaki üst düzey yorumların sayısı. **/
    commentCount: number
    /** Bu sayfa için canlı güncellemeler aboneliğinde kullanılan kimlik. **/
    urlIdWS: string
    /** En yeni önce. Yalnızca includeUserInfo gönderildiğinde ayarlanır. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]