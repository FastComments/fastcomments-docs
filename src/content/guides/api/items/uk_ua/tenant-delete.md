[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Цей маршрут дозволяє видалити `Tenant` **та всі пов'язані дані** (користувачі, коментарі тощо) за id.

Існують такі обмеження щодо видалення тенантів:

- Тенант має належати вам або бути white-label тенантом, яким ви керуєте.
- Параметр запиту `sure` має бути встановлений у `true`.

[inline-code-attrs-start title = 'Приклад cURL для видалення тенанта'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту для видалення тенанта'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді при видаленні тенанта'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Включається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Включається у разі невдачі. **/
    reason?: string
}
[inline-code-end]