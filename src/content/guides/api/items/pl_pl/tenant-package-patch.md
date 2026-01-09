[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Ten endpoint API umożliwia zaktualizowanie `TenantPackage` według `id`.

Aktualizacja `TenantPackage` ma następujące ograniczenia:

- Jeśli ustawiasz `hasFlexPricing` na true, to wszystkie parametry `flex*` są wymagane w tym samym żądaniu.
- `name` nie może być dłuższy niż `50 characters`.
- Każdy element `forWhoText` nie może być dłuższy niż `200 characters`.
- Każdy element `featureTaglines` nie może być dłuższy niż `100 characters`.
- `TenantPackage` musi być "mniejszy" niż parent tenant. Na przykład wszystkie parametry `max*` muszą mieć mniejsze wartości niż parent tenant. 
- Nie możesz zmienić `tenantId` powiązanego z `TenantPackage`.

[inline-code-attrs-start title = 'Przykład cURL dla PATCH TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania PATCH TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi PATCH TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]

---