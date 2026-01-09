[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет возможность добавить один `Tenant`.

При создании `Tenant` действуют следующие ограничения:

- Поле `name` обязательно.
- Поле `domainConfiguration` обязательно.
- При создании `Tenant` нельзя указывать следующие значения:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- Значение `signUpDate` не может быть в будущем.
- Длина `name` не может превышать `200 characters`.
- Длина `email` не может превышать `300 characters`.
- `email` должен быть уникален среди всех tenants FastComments.com.
- Вы не можете создавать tenants, если у родительского tenant не определён действительный `TenantPackage`.
  - Если ваш tenant был создан через FastComments.com, это не должно быть проблемой.
- Вы не можете создать больше tenants, чем указано в `maxWhiteLabeledTenants` в вашем пакете.
- Вы должны указать query-параметр `tenantId`, который является id вашего `parent tenant` с включенным white labeling.

Мы можем создать `Tenant` с помощью всего нескольких параметров:

[inline-code-attrs-start title = 'Пример cURL-запроса создания Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запроса для создания Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при создании Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Указывается при ошибке. **/
    reason?: string
    tenant?: Tenant; // При успехе возвращается полностью созданный tenant.
}
[inline-code-end]