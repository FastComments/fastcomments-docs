[api-resource-header-start name = 'User'; route = 'GET /api/v1/users/:id'; creditsCost = 1; api-resource-header-end]

Этот маршрут возвращает одного User по id.

[inline-code-attrs-start title = 'Пример cURL-запроса по ID пользователя'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса пользователя'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UserByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа пользователя'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UserByIdResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'unauthorized'
    /** Включается при ошибке. **/
    reason?: string
    user?: User
}
[inline-code-end]