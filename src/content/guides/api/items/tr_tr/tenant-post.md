[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir `Tenant` ekleme yeteneği sağlar.

Bir `Tenant` oluşturmanın aşağıdaki kısıtlamaları vardır:

- A `name` is required.
- `domainConfiguration` is required.
- The following values may not be provided when creating a `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- The `signUpDate` may not be in the future.
- The `name` may not be longer than `200 characters`.
- The `email` may not be longer than `300 characters`.
- The `email` must be unique across all of FastComments.com tenants.
- You may not create tenants if the parent tenant does not have a valid `TenantPackage` defined.
  - If your tenant was created via FastComments.com, this shouldn't be an issue.
- You may not create more tenants than defined under `maxWhiteLabeledTenants` in your package.
- You must specify the `tenantId` query param which is the id of your `parent tenant` with white labeling enabled.

Sadece birkaç parametre ile bir `Tenant` oluşturabiliriz:

[inline-code-attrs-start title = 'Tenant Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Tenant Oluşturma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Oluşturma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Hata durumunda dahil edilir. **/
    reason?: string
    tenant?: Tenant; // Başarı durumunda oluşturulan tenant'ın tamamını döndürürüz.
}
[inline-code-end]