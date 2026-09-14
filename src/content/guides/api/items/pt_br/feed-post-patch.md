[api-resource-header-start name = 'FeedPost'; route = 'PATCH /api/v1/feed-posts/:id'; creditsCost = 1; api-resource-header-end]

Esta rota atualiza um único `FeedPost`. Envie apenas os campos que você deseja alterar.

[inline-code-attrs-start title = 'Exemplo cURL de Atualização de FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/feed-posts/some-post-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "title": "Release 2.0.1 is out",
    "tags": ["releases", "hotfix"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Atualização de FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchQueryParams {
    tenantId: string
    API_KEY: string
}

/** Qualquer campo gravável da estrutura FeedPost. **/
type FeedPostPatchBody = Partial<Pick<FeedPost, 'title' | 'contentHTML' | 'fromUserDisplayName' | 'tags' | 'weight' | 'meta' | 'media' | 'links'>>
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Atualização de FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'internal'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]