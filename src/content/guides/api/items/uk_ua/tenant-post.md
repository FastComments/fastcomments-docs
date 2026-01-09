[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Цей маршрут надає можливість додати один `Tenant`.

Створення `Tenant` має такі обмеження:

- Поле `name` є обов'язковим.
- Поле `domainConfiguration` є обов'язковим.
- Наступні значення не можуть бути вказані під час створення `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- Поле `signUpDate` не може бути в майбутньому.
- Поле `name` не може бути довшим за `200 characters`.
- Поле `email` не може бути довшим за `300 characters`.
- Поле `email` має бути унікальним серед усіх tenants FastComments.com.
- Ви не можете створювати tenants, якщо батьківський tenant не має визначеного дійсного `TenantPackage`.
  - Якщо ваш tenant був створений через FastComments.com, це не має становити проблему.
- Ви не можете створити більше tenants, ніж визначено у `maxWhiteLabeledTenants` у вашому пакеті.
- Ви повинні вказати параметр запиту `tenantId`, який є id вашого `parent tenant` з увімкненою white labeling.

Ми можемо створити `Tenant`, вказавши лише декілька параметрів:

[inline-code-attrs-start title = 'Приклад cURL для створення Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запиту для створення Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді для створення Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Додається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Додається у разі помилки. **/
    reason?: string
    tenant?: Tenant; // Ми повертаємо повний створений Tenant у разі успіху.
}
[inline-code-end]