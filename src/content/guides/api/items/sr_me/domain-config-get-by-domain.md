[api-resource-header-start name = 'DomainConfig'; route = 'GET /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Појединачне DomainConfigs се могу преузети по одговарајућем `domain`. 

[inline-code-attrs-start title = 'cURL пример: Конфигурација домена по домену'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за конфигурацију домена по домену'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigsByDomainRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за конфигурацију домена по домену'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'internal' | 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-domain' | 'update-would-create-duplicate' | 'domain-does-not-exist'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    configuration?: DomainConfig | null
}
[inline-code-end]