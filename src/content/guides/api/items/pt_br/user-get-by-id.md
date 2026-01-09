[api-resource-header-start name = 'User'; route = 'GET /api/v1/users/:id'; creditsCost = 1; api-resource-header-end]

Esta rota retorna um único User por id.

[inline-code-attrs-start title = 'Exemplo cURL de User por ID'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da requisição do User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UserByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de resposta do User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UserByIdResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'unauthorized'
    /** Incluído em caso de falha. **/
    reason?: string
    user?: User
}
[inline-code-end]

---