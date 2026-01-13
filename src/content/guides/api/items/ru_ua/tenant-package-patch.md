[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Эта конечная точка API предоставляет возможность обновить `TenantPackage` по `id`.

Обновление `TenantPackage` имеет следующие ограничения:

- Если вы устанавливаете `hasFlexPricing` в true, то все параметры `flex*` обязательны в том же запросе.
- Значение `name` не может быть длиннее, чем `50 characters`.
- Каждый элемент `forWhoText` не может быть длиннее, чем `200 characters`.
- Каждый элемент `featureTaglines` не может быть длиннее, чем `100 characters`.
- `TenantPackage` должен быть "меньше", чем родительский tenant. Например, все параметры `max*` должны иметь значения меньше, чем у родительского tenant. 
- Вы не можете изменить `tenantId`, связанный с `TenantPackage`.

[inline-code-attrs-start title = 'Пример cURL-запроса TenantPackage PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса TenantPackage PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа TenantPackage PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]

---