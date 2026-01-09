[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Questo endpoint API fornisce la possibilità di aggiornare un `TenantPackage` tramite `id`.

L'aggiornamento di un `TenantPackage` ha le seguenti restrizioni:

- Se si imposta `hasFlexPricing` su true, allora tutti i parametri `flex*` sono obbligatori nella stessa richiesta.
- Il `name` non può essere più lungo di `50 characters`.
- Ogni elemento `forWhoText` non può essere più lungo di `200 characters`.
- Ogni elemento `featureTaglines` non può essere più lungo di `100 characters`.
- Il `TenantPackage` deve essere "più piccolo" del tenant principale. Ad esempio, tutti i parametri `max*` devono avere valori inferiori rispetto al tenant principale.
- Non è possibile modificare il `tenantId` associato a un `TenantPackage`.

[inline-code-attrs-start title = 'Esempio cURL PATCH per TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta PATCH TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta PATCH TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]

---