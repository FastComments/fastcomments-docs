[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Ta trasa umożliwia dodanie pojedynczego `Tenant`.

Tworzenie `Tenant` podlega następującym ograniczeniom:

- Wymagane jest pole `name`.
- Wymagane jest pole `domainConfiguration`.
- Następujące wartości nie mogą być podane podczas tworzenia `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- `signUpDate` nie może być w przyszłości.
- `name` nie może być dłuższe niż `200 characters`.
- `email` nie może być dłuższe niż `300 characters`.
- `email` musi być unikalny wśród wszystkich tenantów FastComments.com.
- Nie możesz tworzyć tenantów, jeśli tenant nadrzędny nie ma zdefiniowanego prawidłowego `TenantPackage`.
  - Jeśli Twój tenant został utworzony za pośrednictwem FastComments.com, nie powinno to stanowić problemu.
- Nie możesz utworzyć więcej tenantów niż zdefiniowano w `maxWhiteLabeledTenants` w Twoim pakiecie.
- Musisz określić parametr zapytania `tenantId`, który jest identyfikatorem Twojego `parent tenant` z włączonym white labelingiem.

Możemy utworzyć `Tenant` podając tylko kilka parametrów:

[inline-code-attrs-start title = 'Przykład cURL tworzenia Tenanta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura żądania tworzenia Tenanta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi tworzenia Tenanta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Dołączane w przypadku niepowodzenia. **/
    reason?: string
    tenant?: Tenant; // Zwracamy kompletnie utworzony tenant w przypadku powodzenia.
}
[inline-code-end]

---