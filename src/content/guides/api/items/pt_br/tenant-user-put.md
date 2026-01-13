[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Esta rota permite substituir um único `TenantUser`.

Substituir um `TenantUser` tem as seguintes restrições:

- O `signUpDate` não pode estar no futuro.
- O `locale` deve estar na lista de [Localidades Suportadas](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- O `username` deve ser único em todo o FastComments.com. Se isso for um problema, sugerimos usar SSO.
- O `email` deve ser único em todo o FastComments.com. Se isso for um problema, sugerimos usar SSO.
- Você não pode atualizar o `tenantId` de um usuário.

Podemos criar um `TenantUser` da seguinte forma

[inline-code-attrs-start title = 'Exemplo cURL de substituição de TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Substituição de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Quando email ou username forem alterados, você pode definir isto para true para também atualizar os comentários do usuário. Isso dobrará o custo em créditos. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Substituição de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]

---