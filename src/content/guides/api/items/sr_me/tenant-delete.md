[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Ова рута омогућава уклањање `Tenant` **и свих повезаних података** (корисници, коментари, итд.) по id.

Постоје следећа ограничења у вези са брисањем tenant-а:

- Tenant мора бити ваш, или брандирани tenant који ви управљате.
- Параметар упита `sure` мора бити постављен на `true`.

[inline-code-attrs-start title = 'Пример cURL захтева за уклањање Tenant-а'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за уклањање Tenant-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање Tenant-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Укључено у случају неуспјеха. **/
    reason?: string
}
[inline-code-end]