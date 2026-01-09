[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Este endpoint da API fornece a capacidade de criar uma `Subscription`. Note que um usuário pode ter apenas uma assinatura por página, pois mais de uma seria redundante, e tentar
criar mais de uma assinatura para o mesmo usuário na mesma página resultará em um erro.

Ao criar uma assinatura, objetos `Notification` serão criados quando um novo comentário for deixado na raiz do `urlId` assinado (quando o `parentId` do comentário for `null`).

[inline-code-attrs-start title = 'Exemplo cURL de POST para Subscription'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "userId": "tenantId:test-user-id",
    "urlId": "some-page-id-or-url",
    "url": "https://example.com/page",
    "pageTitle": "Some Example Page!"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da requisição POST de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da resposta POST de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]