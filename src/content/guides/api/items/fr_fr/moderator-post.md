[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Cette route fournit la capacité d'ajouter un seul `Moderator`.

La création d'un `Moderator` a les restrictions suivantes :

- Un `name` et un `email` doivent toujours être fournis. Un `userId` est optionnel.
- Les valeurs suivantes ne peuvent pas être fournies lors de la création d'un `Moderator` :
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

Nous pouvons créer un `Moderator` pour un utilisateur dont nous ne connaissons que l'email :

[inline-code-attrs-start title = 'Exemple cURL de Création de Modérateur via Email'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Ou nous pouvons créer un `Moderator` pour un utilisateur qui appartient à notre locataire, pour suivre leurs statistiques de modération :

[inline-code-attrs-start title = 'Exemple cURL de Création de Modérateur via Utilisateur Locataire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Création de Modérateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Création de Modérateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    moderator?: Moderator; // We return the complete created moderator on success.
}
[inline-code-end]
