[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Esta rota permite atualizar um único `TenantUser`.

A atualização de um `TenantUser` tem as seguintes restrições:

- O `signUpDate` não pode estar no futuro.
- O `locale` deve constar na lista de [Locales Suportados](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- O `username` deve ser único em todo o FastComments.com. Se isso for um problema, sugerimos usar SSO.
- O `email` deve ser único em todo o FastComments.com. Se isso for um problema, sugerimos usar SSO.
- Você não pode atualizar o `tenantId` de um usuário.

Podemos criar um `TenantUser` da seguinte forma

[inline-code-attrs-start title = 'Exemplo de cURL de Atualização de TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Atualização de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Quando o email ou username for alterado, você pode definir isto como true para também atualizar os comentários do usuário. Isso dobrará o custo em créditos. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Atualização de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

---