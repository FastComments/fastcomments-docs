[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/domain-configs'; creditsCost = 1; api-resource-header-end]

Этот API предоставляет возможность получить все объекты `DomainConfig` для тенанта.

[inline-code-attrs-start title = 'Пример cURL-запроса для DomainConfig (GET)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/domain-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса DomainConfig (GET)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetDomainConfigsRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа DomainConfig (GET)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetDomainConfigsResponse {
    status: 'success' | 'failed'
    /** Присутствует при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Присутствует при ошибке. **/
    reason?: string
    /** Конфигурации! **/
    configurations: DomainConfig[] | null
}
[inline-code-end]

---