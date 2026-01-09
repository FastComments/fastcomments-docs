[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Esta rota retorna até 30 objetos `Subscription` ordenados por `createdAt`, do mais recente para o mais antigo.

Você pode filtrar por `userId`. Com SSO, o id do usuário está no formato `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Exemplo cURL — Assinaturas para Usuário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição — Assinaturas'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginar pulando registros. **/
    skip?: number
    /** Filtrar por usuário. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta — Assinaturas'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Incluído em caso de falha. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]

---