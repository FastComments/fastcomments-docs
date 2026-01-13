[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Цей API-ендпойнт надає можливість оновити `TenantPackage` за `id`.

Оновлення `TenantPackage` має такі обмеження:

- Якщо ви встановлюєте `hasFlexPricing` в true, тоді всі параметри `flex*` повинні бути вказані в тому ж запиті.
- Поле `name` не може бути довшим за `50 characters`.
- Кожен елемент `forWhoText` не може бути довшим за `200 characters`.
- Кожен елемент `featureTaglines` не може бути довшим за `100 characters`.
- `TenantPackage` має бути "меншим" за батьківський tenant. Наприклад, всі параметри `max*` повинні мати менші значення, ніж у батьківського tenant. 
- Ви не можете змінювати `tenantId`, пов'язаний з `TenantPackage`.

[inline-code-attrs-start title = 'Приклад cURL для PATCH TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту PATCH TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді PATCH TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Включається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Включається у разі невдачі. **/
    reason?: string
}
[inline-code-end]

---