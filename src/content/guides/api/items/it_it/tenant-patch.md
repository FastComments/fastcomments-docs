[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Questo endpoint API permette di aggiornare un `Tenant` tramite il suo `id`.

L'aggiornamento di un `Tenant` è soggetto alle seguenti restrizioni:

- I seguenti valori non possono essere aggiornati:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- La `signUpDate` non può essere nel futuro.
- La `name` non può essere più lunga di `200 characters`.
- La `email` non può essere più lunga di `300 characters`.
- La `email` deve essere unica tra tutti i tenant di FastComments.com.
- Quando si imposta `billingInfoValid` a `true`, `billingInfo` deve essere fornito nella stessa richiesta.
- Non è possibile aggiornare il `packageId` associato al proprio tenant.
- Non è possibile aggiornare il `paymentFrequency` associato al proprio tenant.

[inline-code-attrs-start title = 'Esempio cURL PATCH Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta PATCH Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta PATCH Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Incluso in caso di fallimento. **/
    reason?: string
}
[inline-code-end]

---