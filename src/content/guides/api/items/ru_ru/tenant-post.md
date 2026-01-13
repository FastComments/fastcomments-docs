[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет возможность добавить одного `Tenant`.

Создание `Tenant` имеет следующие ограничения:

- Обязателен `name`.
- Обязателен `domainConfiguration`.
- Следующие значения не могут быть указаны при создании `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- Значение `signUpDate` не может быть в будущем.
- `name` не может быть длиннее `200 characters`.
- `email` не может быть длиннее `300 characters`.
- `email` должен быть уникальным для всех тенантов FastComments.com.
- Вы не можете создавать тенанты, если у родительского тенанта не определён действительный `TenantPackage`.
  - Если ваш тенант был создан через FastComments.com, это не должно быть проблемой.
- Вы не можете создавать больше тенантов, чем определено в `maxWhiteLabeledTenants` в вашем пакете.
- Вы должны указать параметр запроса `tenantId`, который является id вашего `parent tenant` с включенным white labeling.

Мы можем создать `Tenant` с помощью лишь нескольких параметров:

[inline-code-attrs-start title = 'Пример cURL-запроса для создания Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    tenant?: Tenant; // При успешном выполнении возвращаем полностью созданный tenant.
}
[inline-code-end]

---