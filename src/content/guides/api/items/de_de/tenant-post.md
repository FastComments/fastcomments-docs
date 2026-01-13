[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Diese Route bietet die Möglichkeit, einen einzelnen `Tenant` hinzuzufügen.

Das Erstellen eines `Tenant` hat die folgenden Einschränkungen:

- Ein `name` ist erforderlich.
- `domainConfiguration` ist erforderlich.
- Die folgenden Werte dürfen beim Erstellen eines `Tenant` nicht angegeben werden:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
- Das `signUpDate` darf nicht in der Zukunft liegen.
- Der `name` darf nicht länger als `200 Zeichen` sein.
- Die `email` darf nicht länger als `300 Zeichen` sein.
- Die `email` muss über alle FastComments.com-Tenants hinweg eindeutig sein.
- Sie können keine Tenants erstellen, wenn der übergeordnete Tenant kein gültiges `TenantPackage` definiert hat.
  - Wenn Ihr Tenant über FastComments.com erstellt wurde, sollte dies kein Problem sein.
- Sie können nicht mehr Tenants erstellen als unter `maxWhiteLabeledTenants` in Ihrem Paket definiert.
- Sie müssen den `tenantId`-Abfrageparameter angeben, der die ID Ihres `übergeordneten Tenants` mit aktiviertem White Labeling ist.

Wir können einen `Tenant` mit nur wenigen Parametern erstellen:

[inline-code-attrs-start title = 'Tenant Erstellen cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Tenant Erstellen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Erstellen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Included on failure. **/
    reason?: string
    tenant?: Tenant; // We return the complete created tenant on success.
}
[inline-code-end]
