[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Този маршрут предоставя премахване на `Tenant` **и всички свързани данни** (потребители, коментари и т.н.) по id.

Съществуват следните ограничения около премахването на tenant-и:

- Tenant-ът трябва да е ваш собствен или white labeled tenant, който управлявате.
- Параметърът на заявката `sure` трябва да е зададен на `true`.

[inline-code-attrs-start title = 'Пример за премахване на Tenant с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за премахване на Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за премахване на Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
