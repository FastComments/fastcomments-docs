[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Bir sayfadaki her reaksiyonun sayısını ve isteğe bağlı olarak reaksiyon gösteren kullanıcıları, reaksiyon kimliğiyle bir giriş olarak, en yeni önce olacak şekilde döndürür. Bu, [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) isteğinin sunucu tarafı versiyonudur.

`includeUserInfo` hangi kullanıcıların döndürüleceğini kontrol eder:

- `public` profil etkinliğini gizli yapan kullanıcıları dışarı bırakır.
- `include-private` onları dahil eder. Bu seçeneği yalnızca liste halka gösterilmeyecekse kullanın.

Anonim kullanıcılar hiçbir zaman dahil edilmez ve en fazla 100 reaksiyon aranır. Var olmayan sayfalar `counts` döndürmez ve boş bir `users` listesi verir.

[inline-code-attrs-start title = 'Sayfa Tepkileri cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Tepkileri İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Yalnızca sayıları almak için atlayın. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Tepkileri Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    /** Reaksiyon kimliğine göre sayı, örneğin {\"heart\": 12, \"laugh\": 3}. Sayfada reaksiyon yoksa ayarlanmaz. **/
    counts?: Record<string, number>
    /** Her reaksiyon için bir giriş, en yeni önce. Yalnızca includeUserInfo gönderildiğinde ayarlanır. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** Reaksiyon kimliği, örneğin \"heart\". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Faydalı İpucu

`urlId` gibi değerleri URI Encode etmeyi unutmayın.