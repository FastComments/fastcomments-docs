[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Bu API, bir yorumu id ile almanızı sağlar.

[inline-code-attrs-start title = 'Yorumu ID ile Getirme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Yorumu ID ile Getirme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsGetByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorumu ID ile Getirme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentGetByIdResponse {
    status: 'success' | 'failed'
    /** Başarısızlık halinde dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'missing-id'
    /** Başarısızlık halinde dahil edilir. **/
    reason?: string
    /** Yorum! **/
    comment?: Comment
}
[inline-code-end]

---