[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint de API fornece a capacidade de atualizar um `Tenant` pelo `id`.

A atualização de um `Tenant` tem as seguintes restrições:

- Os seguintes valores não podem ser atualizados:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- O `signUpDate` não pode estar no futuro.
- O `name` não pode ter mais de `200 characters`.
- O `email` não pode ter mais de `300 characters`.
- O `email` deve ser único entre todos os tenants do FastComments.com.
- Ao definir `billingInfoValid` como `true`, `billingInfo` deve ser fornecido na mesma requisição.
- Você não pode atualizar o `packageId` associado ao seu próprio tenant.
- Você não pode atualizar o `paymentFrequency` associado ao seu próprio tenant.

[inline-code-attrs-start title = 'Exemplo de cURL PATCH para Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição PATCH do Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta PATCH do Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]