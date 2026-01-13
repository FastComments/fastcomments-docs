[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Цей маршрут дозволяє надіслати посилання для входу одному `TenantUser`.

Корисно при масовому створенні користувачів, щоб не інструктувати їх щодо входу на FastComments.com. Це надішле їм «магічне посилання» для входу, яке діє протягом `30 days`.

Існують наступні обмеження для надсилання посилання на вхід `TenantUser`:
- The `TenantUser` must already exist.
- You must have access to manage the `Tenant` the `TenantUser` belongs to.

Ми можемо надіслати посилання для входу `TenantUser` таким чином:

[inline-code-attrs-start title = 'Приклад cURL запиту для посилання входу TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

This will send an email like `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Структура запиту посилання входу TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді на запит посилання входу TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Включено у випадку помилки. **/
    reason?: string
}
[inline-code-end]