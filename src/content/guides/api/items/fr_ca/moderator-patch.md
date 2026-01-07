[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de mettre à jour un `Moderator` par `id`.

La mise à jour d'un `Moderator` a les restrictions suivantes :

- Les valeurs suivantes ne peuvent pas être fournies lors de la mise à jour d'un `Moderator` :
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Lorsqu'un `userId` est spécifié, cet utilisateur doit exister.
- Lorsqu'un `userId` est spécifié, il doit appartenir au même `tenantId` spécifié dans les paramètres de requête.
- Deux modérateurs dans le même locataire ne peuvent pas être ajoutés avec le même `email`.
- Vous ne pouvez pas changer le `tenantId` associé à un `Moderator`.

[inline-code-attrs-start title = 'Exemple cURL de PATCH de Modérateur'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête PATCH de Modérateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse PATCH de Modérateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
