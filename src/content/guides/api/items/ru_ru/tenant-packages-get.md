---
[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Этот API использует пагинацию, осуществляемую с помощью параметра запроса `skip`. TenantPackages возвращаются страницами по `100`, упорядоченные по `createdAt` и `id`.

Стоимость рассчитывается на основе количества возвращаемых tenant packages и составляет `1 credit per 10` tenant packages.

[inline-code-attrs-start title = 'Пример cURL для TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Количество tenant packages, которые нужно пропустить для пагинации. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Присутствует в случае ошибки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Присутствует в случае ошибки. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]

---