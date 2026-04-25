[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası bir yorumu silme yeteneği sağlar.

Notlar:

- Bu API istenirse yorum bileşenini "canlı" olarak güncelleyebilir (bu `creditsCost`'u `1`'den `2`'ye çıkarır).
- Bu API tüm alt yorumları silecektir.
- Hedef yorum kilitliyse (`isLocked: true`), istek `code: 'locked'` ile reddedilir. Önce yorumu kilidini açın, sonra silin.

[inline-code-attrs-start title = 'Yorum Silme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Yorum Silme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Güncellemeyi yapan kullanıcı. İstenirse, yorumı silme yetkisine sahip olup olmadığını kontrol etmek için kullanılabilir.  **/
    contextUserId?: string
	/** Yorumun aynı urlId ile yorum bileşeninin örneklerini görüntüleyen kullanıcılara "canlı" olarak silinip silinmeyeceği. NOT: Kredi maliyetini `1`'den `2`'ye katlar. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorum Silme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]