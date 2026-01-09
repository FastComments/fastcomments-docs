[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Этот API использует пагинацию, задаваемую параметром запроса `skip`. TenantUsers возвращаются страницами по `100`, упорядоченные по `signUpDate`, `username` и `id`.

Стоимость основывается на количестве возвращённых TenantUsers — `1 кредит за 10` возвращённых пользователей.

[inline-code-attrs-start title = 'Пример cURL-запроса TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Количество пользователей арендатора, которые нужно пропустить для пагинации. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включается при ошибке. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---