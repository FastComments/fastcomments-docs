[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Esta rota fornece a capacidade de enviar um link de login para um único `TenantUser`.

Útil ao criar usuários em lote e não precisar instruí-los sobre como fazer login no FastComments.com. Isso simplesmente enviará um "link mágico" para login que expira após `30 days`.

As seguintes restrições se aplicam para enviar um link de login a um `TenantUser`:
- O `TenantUser` já deve existir.
- Você deve ter acesso para gerenciar o `Tenant` ao qual o `TenantUser` pertence.

Podemos enviar um link de login para um `TenantUser` da seguinte maneira:

[inline-code-attrs-start title = 'Exemplo cURL do link de login do TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Isso enviará um e-mail como `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Estrutura de Requisição do Link de Login do TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta do Link de Login do TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]