[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users/by-email/:email'; creditsCost = 1; api-resource-header-end]

Esta rota retorna um único usuário SSO pelo seu e-mail.

[inline-code-attrs-start title = 'Exemplo cURL de SSOUser por e-mail'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users/by-email/someone@somewhere.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserRequestByEmailQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserByEmailResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-email' | 'user-does-not-exist'
    /** Incluído em caso de falha. **/
    reason?: string
    user: SSOUser
}
[inline-code-end]

---