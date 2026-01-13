[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Esta ruta proporciona la capacidad de agregar un único `Tenant`.

Crear un `Tenant` tiene las siguientes restricciones:

- Se requiere un `name`.
- Se requiere `domainConfiguration`.
- Los siguientes valores no pueden ser proporcionados al crear un `Tenant`:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
- El `signUpDate` no puede estar en el futuro.
- El `name` no puede tener más de `200 caracteres`.
- El `email` no puede tener más de `300 caracteres`.
- El `email` debe ser único en todos los inquilinos de FastComments.com.
- No puede crear inquilinos si el inquilino padre no tiene un `TenantPackage` válido definido.
  - Si su inquilino fue creado a través de FastComments.com, esto no debería ser un problema.
- No puede crear más inquilinos de los definidos en `maxWhiteLabeledTenants` en su paquete.
- Debe especificar el parámetro de consulta `tenantId` que es el id de su `inquilino padre` con marca blanca habilitada.

Podemos crear un `Tenant` con solo unos pocos parámetros:

[inline-code-attrs-start title = 'Ejemplo cURL de Creación de Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Creación de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Creación de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Included on failure. **/
    reason?: string
    tenant?: Tenant; // We return the complete created tenant on success.
}
[inline-code-end]
