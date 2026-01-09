[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Questa API fornisce la possibilità di recuperare un singolo commento tramite id.

[inline-code-attrs-start title = 'Esempio cURL - Recupero commento per ID'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta - Recupero commento per ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsGetByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta - Recupero commento per ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentGetByIdResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'missing-id'
    /** Incluso in caso di errore. **/
    reason?: string
    /** Il commento! **/
    comment?: Comment
}
[inline-code-end]