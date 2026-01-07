[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за актуализиране на `TenantPackage` по `id`.

Актуализирането на `TenantPackage` има следните ограничения:

- Ако задавате `hasFlexPricing` на true, тогава всички `flex*` параметри са задължителни в същата заявка.
- `name` не може да бъде по-дълго от `50 символа`.
- Всеки елемент на `forWhoText` не може да бъде по-дълъг от `200 символа`.
- Всеки елемент на `featureTaglines` не може да бъде по-дълъг от `100 символа`.
- `TenantPackage` трябва да бъде "по-малък" от родителския tenant. Например, всички `max*` параметри трябва да имат по-ниски стойности от родителския tenant.
- Не можете да променяте `tenantId`, асоцииран с `TenantPackage`.

[inline-code-attrs-start title = 'Пример за PATCH на TenantPackage с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за PATCH на TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за PATCH на TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
