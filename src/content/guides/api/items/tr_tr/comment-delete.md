[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası bir yorumu silme yeteneği sağlar.

Notlar:

- İstenirse bu API yorum bileşenini "live" olarak güncelleyebilir (bu, `creditsCost` değerini `1`'den `2`'ye çıkarır).
- Bu API tüm alt yorumları siler.

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
	/** Güncellemeyi yapan kullanıcı. İstenirse bu, kullanıcının yorumu silebileceğini kontrol etmek için kullanılabilir.  **/
    contextUserId?: string
	/** Yorumun aynı urlId'ye sahip yorum bileşeni örneklerini görüntüleyen kullanıcılara "live" olarak silinip silinmeyeceği. NOT: Kredi maliyetini 1'den 2'ye iki katına çıkarır. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorum Silme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]

---