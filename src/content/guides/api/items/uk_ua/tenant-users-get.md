[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Цей API використовує пагінацію, що забезпечується параметром запиту `skip`. TenantUsers повертаються сторінками по `100`, впорядковані за `signUpDate`, `username` та `id`.

Вартість базується на кількості повернутих користувачів тенанта, становить `1 credit per 10` повернутих користувачів тенанта.

[inline-code-attrs-start title = 'Приклад cURL для TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Кількість користувачів тенанта, які слід пропустити для пагінації. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Додається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Додається у разі помилки. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---