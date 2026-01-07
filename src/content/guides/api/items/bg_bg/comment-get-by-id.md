[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Този API предоставя възможност за извличане на единичен коментар по id.

[inline-code-attrs-start title = 'Пример за получаване на коментар по Id с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за получаване на коментар по Id'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsGetByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за получаване на коментар по Id'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentGetByIdResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'missing-id'
    /** Included on failure. **/
    reason?: string
    /** The comment! **/
    comment?: Comment
}
[inline-code-end]
