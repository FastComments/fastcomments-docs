[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de actualizar un `TenantPackage` por `id`.

Actualizar un `TenantPackage` tiene las siguientes restricciones:

- Si está estableciendo `hasFlexPricing` a verdadero, entonces todos los parámetros `flex*` son requeridos en esa misma solicitud.
- El `name` no puede tener más de `50 caracteres`.
- Cada elemento de `forWhoText` no puede tener más de `200 caracteres`.
- Cada elemento de `featureTaglines` no puede tener más de `100 caracteres`.
- El `TenantPackage` debe ser "más pequeño" que el inquilino padre. Por ejemplo, todos los parámetros `max*` deben tener valores más bajos que el inquilino padre.
- No puede cambiar el `tenantId` asociado con un `TenantPackage`.

[inline-code-attrs-start title = 'Ejemplo cURL de PATCH de TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de PATCH de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de PATCH de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
