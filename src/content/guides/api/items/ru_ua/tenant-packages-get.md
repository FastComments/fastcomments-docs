[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Этот API использует постраничную навигацию, реализованную через параметр запроса `skip`. TenantPackages возвращаются страницами по `100`, упорядоченных по `createdAt` и `id`.

Стоимость основана на количестве возвращённых TenantPackages, составляя `1 credit per 10` возвращённых TenantPackages.

[inline-code-attrs-start title = 'Пример cURL запроса TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Количество TenantPackages, которые нужно пропустить для пагинации. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Указывается при ошибке. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]

---