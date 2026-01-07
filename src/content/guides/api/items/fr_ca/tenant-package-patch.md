[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de mettre à jour un `TenantPackage` par `id`.

La mise à jour d'un `TenantPackage` a les restrictions suivantes:

- Si vous définissez `hasFlexPricing` à true, alors tous les paramètres `flex*` sont requis dans cette même requête.
- Le `name` ne peut pas dépasser `50 caractères`.
- Chaque élément `forWhoText` ne peut pas dépasser `200 caractères`.
- Chaque élément `featureTaglines` ne peut pas dépasser `100 caractères`.
- Le `TenantPackage` doit être "plus petit" que le locataire parent. Par exemple, tous les paramètres `max*` doivent avoir des valeurs inférieures à celles du locataire parent.
- Vous ne pouvez pas changer le `tenantId` associé à un `TenantPackage`.

[inline-code-attrs-start title = 'Exemple cURL de PATCH de TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête PATCH de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse PATCH de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
