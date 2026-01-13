[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, einen `Tenant` nach `id` zu aktualisieren.

Das Aktualisieren eines `Tenant` hat die folgenden Einschränkungen:

- Die folgenden Werte können nicht aktualisiert werden:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
  - `managedByTenantId`
- Das `signUpDate` darf nicht in der Zukunft liegen.
- Der `name` darf nicht länger als `200 Zeichen` sein.
- Die `email` darf nicht länger als `300 Zeichen` sein.
- Die `email` muss über alle FastComments.com-Tenants hinweg eindeutig sein.
- Wenn `billingInfoValid` auf `true` gesetzt wird, muss `billingInfo` in derselben Anfrage angegeben werden.
- Sie können die `packageId`, die mit Ihrem eigenen Tenant verknüpft ist, nicht aktualisieren.
- Sie können die `paymentFrequency`, die mit Ihrem eigenen Tenant verknüpft ist, nicht aktualisieren.

[inline-code-attrs-start title = 'Tenant PATCH cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
