---
[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Esta rota fornece a capacidade de adicionar um único `Tenant`.

Criar um `Tenant` tem as seguintes restrições:

- Um `name` é obrigatório.
- `domainConfiguration` é obrigatório.
- Os seguintes valores não podem ser fornecidos ao criar um `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- O `signUpDate` não pode estar no futuro.
- O `name` não pode ter mais do que `200 characters`.
- O `email` não pode ter mais do que `300 characters`.
- O `email` deve ser único entre todos os tenants do FastComments.com.
- Você não pode criar tenants se o tenant pai não tiver um `TenantPackage` válido definido.
  - Se o seu tenant foi criado via FastComments.com, isso não deve ser um problema.
- Você não pode criar mais tenants do que o definido em `maxWhiteLabeledTenants` no seu pacote.
- Você deve especificar o parâmetro de query `tenantId`, que é o id do seu `parent tenant` com white labeling habilitado.

Podemos criar um `Tenant` com apenas alguns parâmetros:

[inline-code-attrs-start title = 'Exemplo cURL de criação de Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Criação de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Criação de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Incluído em caso de falha. **/
    reason?: string
    tenant?: Tenant; // Retornamos o tenant completo criado em caso de sucesso.
}
[inline-code-end]

---