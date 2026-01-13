[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Cette route fournit la capacité d'ajouter un seul `TenantUser`.

La création d'un `TenantUser` a les restrictions suivantes :

- Un `username` est requis.
- Un `email` est requis.
- Le `signUpDate` ne peut pas être dans le futur.
- Le `locale` doit être dans la liste des [Locales Supportées](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- Le `username` doit être unique parmi tous les FastComments.com. Si c'est un problème, nous suggérons d'utiliser SSO à la place.
- L'`email` doit être unique parmi tous les FastComments.com. Si c'est un problème, nous suggérons d'utiliser SSO à la place.
- Vous ne pouvez pas créer plus d'utilisateurs locataires que défini sous `maxTenantUsers` dans votre forfait.

Nous pouvons créer un `TenantUser` comme suit

[inline-code-attrs-start title = 'Exemple cURL de Création de TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Création de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Création de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Included on failure. **/
    reason?: string
    tenantUser?: TenantUser; // We return the complete created tenant user on success.
}
[inline-code-end]
