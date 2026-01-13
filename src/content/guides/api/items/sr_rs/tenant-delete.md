[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Ова рута омогућава уклањање `Tenant` **и свих повезаних података** (корисника, коментара итд.) по id.

Постоје следећа ограничења приликом уклањања tenant-ова:

- Tenant мора бити ваш, или white labeled tenant којим управљате.
- Параметар упита `sure` мора бити подешен на `true`.

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

[inline-code-attrs-start title = 'Структура одговора при уклањању Tenant-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---