[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Bu API endpoint'i, bir `Tenant` kaydını `id` üzerinden güncelleme yeteneği sağlar.

Bir `Tenant` güncellemenin aşağıdaki kısıtlamaları vardır:

- Aşağıdaki değerler güncellenemez:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` gelecekte bir tarih olamaz.
- `name` değeri `200 karakterden` uzun olamaz.
- `email` değeri `300 karakterden` uzun olamaz.
- `email`, FastComments.com üzerindeki tüm tenant'lar arasında benzersiz olmalıdır.
- `billingInfoValid` değerini `true` olarak ayarlarken, aynı istekte `billingInfo` sağlanmalıdır.
- Kendi tenant'ınızla ilişkili `packageId` değerini güncelleyemezsiniz.
- Kendi tenant'ınızla ilişkili `paymentFrequency` değerini güncelleyemezsiniz.

[inline-code-attrs-start title = 'Tenant PATCH cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]