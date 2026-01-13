[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Този маршрут предоставя възможност за добавяне на единичен `Tenant`.

Създаването на `Tenant` има следните ограничения:

- `name` е задължително.
- `domainConfiguration` е задължително.
- Следните стойности не могат да бъдат предоставени при създаване на `Tenant`:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
- `signUpDate` не може да бъде в бъдещето.
- `name` не може да бъде по-дълго от `200 символа`.
- `email` не може да бъде по-дълго от `300 символа`.
- `email` трябва да бъде уникален за всички tenant-и на FastComments.com.
- Не можете да създавате tenant-и, ако родителският tenant няма дефиниран валиден `TenantPackage`.
  - Ако вашият tenant е бил създаден чрез FastComments.com, това не би трябвало да е проблем.
- Не можете да създавате повече tenant-и от дефинираното под `maxWhiteLabeledTenants` във вашия пакет.
- Трябва да посочите параметъра на заявката `tenantId`, който е id на вашия `родителски tenant` с активиран white labeling.

Можем да създадем `Tenant` само с няколко параметъра:

[inline-code-attrs-start title = 'Пример за създаване на Tenant с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на заявката за създаване на Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за създаване на Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
