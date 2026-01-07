[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Този маршрут предоставя възможност за изпращане на линк за вход на единичен `TenantUser`.

Полезно е при групово създаване на потребители, без да се налага да им обяснявате как да влязат във FastComments.com. Това просто ще им изпрати "магически линк" за вход, който
изтича след `30 дни`.

Съществуват следните ограничения за изпращане на линк за вход на `TenantUser`:
- `TenantUser` трябва вече да съществува.
- Трябва да имате достъп за управление на `Tenant`, към който принадлежи `TenantUser`.

Можем да изпратим линк за вход на `TenantUser` по следния начин:

[inline-code-attrs-start title = 'Пример за линк за вход на TenantUser с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Това ще изпрати имейл като `Bob от TenantName ви кани да бъдете модератор...`

[inline-code-attrs-start title = 'Структура на заявката за линк за вход на TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за линк за вход на TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
