[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Esta rota fornece a capacidade de adicionar um único `TenantUser`.

Criar um `TenantUser` tem as seguintes restrições:

- Um `username` é obrigatório.
- Um `email` é obrigatório.
- O `signUpDate` não pode estar no futuro.
- O `locale` deve estar na lista de [Locales Suportados](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- O `username` deve ser único em todo o FastComments.com. Se isso for um problema, sugerimos usar SSO.
- O `email` deve ser único em todo o FastComments.com. Se isso for um problema, sugerimos usar SSO.
- Você não pode criar mais tenant users do que definido em `maxTenantUsers` no seu pacote. 

Podemos criar um `TenantUser` da seguinte forma

[inline-code-attrs-start title = 'Exemplo cURL de criação de TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição para Criação de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta da Criação de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Incluído em caso de falha. **/
    reason?: string
    tenantUser?: TenantUser; // Retornamos o tenant user completo criado em caso de sucesso.
}
[inline-code-end]

---