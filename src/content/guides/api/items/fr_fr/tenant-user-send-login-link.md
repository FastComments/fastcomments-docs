[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Cette route fournit la capacité d'envoyer un lien de connexion à un seul `TenantUser`.

Utile lors de la création en masse d'utilisateurs sans avoir à leur expliquer comment se connecter à FastComments.com. Cela leur enverra simplement un "lien magique" de connexion qui
expire après `30 jours`.

Les restrictions suivantes existent pour envoyer un lien de connexion à un `TenantUser` :
- Le `TenantUser` doit déjà exister.
- Vous devez avoir accès pour gérer le `Tenant` auquel le `TenantUser` appartient.

Nous pouvons envoyer un lien de connexion à un `TenantUser` comme suit :

[inline-code-attrs-start title = 'Exemple cURL de Lien de Connexion TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Cela enverra un email comme `Bob de TenantName vous invite à être modérateur...`

[inline-code-attrs-start title = 'Structure de Requête de Lien de Connexion TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Lien de Connexion TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
