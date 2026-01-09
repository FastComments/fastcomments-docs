[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава уклањање појединачног `DomainConfig`-а по id-у.

- Напомена: Уклањање `DomainConfig`-а ће опозвати овлашћење те домене за коришћење FastComments-а.
- Напомена: Поновно додавање домене преко UI-ја ће поново креирати објекат (са само попуњеним пољем `domain`).

[inline-code-attrs-start title = 'Пример cURL захтева за уклањање DomainConfig-а'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за уклањање DomainConfig-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање DomainConfig-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---