[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Cette route fournit la capacité de mettre à jour un seul `TenantUser`.

La mise à jour d'un `TenantUser` a les restrictions suivantes :

- Le `signUpDate` ne peut pas être dans le futur.
- Le `locale` doit être dans la liste des [Locales Supportées](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- Le `username` doit être unique parmi tous les FastComments.com. Si c'est un problème, nous suggérons d'utiliser SSO à la place.
- L'`email` doit être unique parmi tous les FastComments.com. Si c'est un problème, nous suggérons d'utiliser SSO à la place.
- Vous ne pouvez pas mettre à jour le `tenantId` d'un utilisateur.

Nous pouvons créer un `TenantUser` comme suit

[inline-code-attrs-start title = 'Exemple cURL de Mise à Jour de TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Mise à Jour de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** When email or username is changed, you can set this to true to also update the user's comments. This will double the credit cost. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Mise à Jour de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
