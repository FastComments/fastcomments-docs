[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Цей маршрут надає можливість надіслати посилання для входу одному `TenantUser`.

Корисно, коли користувачі створюються пакетно і не потрібно пояснювати їм, як увійти на FastComments.com. Це просто надішле їм «магічне посилання» для входу, яке
завершується через `30 days`.

Існують такі обмеження для надсилання посилання для входу `TenantUser`:
- `TenantUser` вже має існувати.
- Ви повинні мати доступ до керування `Tenant`, до якого належить `TenantUser`.

Ми можемо надіслати посилання для входу `TenantUser` так:

[inline-code-attrs-start title = 'Приклад cURL-запиту посилання для входу TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Це надішле електронний лист на кшталт `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Структура запиту посилання для входу TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді посилання для входу TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]