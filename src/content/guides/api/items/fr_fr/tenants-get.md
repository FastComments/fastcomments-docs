[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Cette API retourne les locataires qui sont gérés par votre locataire.

La pagination est fournie par le paramètre de requête `skip`. Les locataires sont retournés par pages de `100`, triés par `signUpDate` et `id`.

Le coût est basé sur le nombre de locataires retournés, coûtant `1 crédit par 10` locataires retournés.

[inline-code-attrs-start title = 'Exemple cURL de Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Vous pouvez définir des paramètres `meta` sur les objets `Tenant` et rechercher des locataires correspondants. Par exemple, pour la clé `someKey` et la valeur meta `some-value`, nous pouvons
construire un objet JSON avec cette paire clé/valeur puis l'encoder en URI comme paramètre de requête pour filtrer :

[inline-code-attrs-start title = 'Exemple cURL de Requête de Tenant par Meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenants to skip for pagination. **/
    skip?: number
    /** Filter by meta params. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]
