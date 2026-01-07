[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Cette route permet la création d'un seul utilisateur SSO.

Essayer de créer deux utilisateurs avec le même ID résultera en une erreur.

[inline-code-attrs-start title = 'Exemple cURL de Création de SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

Dans cet exemple, nous spécifions `groupIds` pour le contrôle d'accès, mais c'est optionnel.

[inline-code-attrs-start title = 'Structure de Requête de Création de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Création de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]

#### Note d'Intégration

Les données passées par l'API peuvent être remplacées simplement en passant un payload HMAC d'utilisateur SSO différent. Par exemple, si
vous définissez un nom d'utilisateur via l'API, mais passez ensuite un différent via le flux SSO au chargement de la page, nous mettrons automatiquement à jour
leur nom d'utilisateur.

Nous ne mettrons pas à jour les paramètres utilisateur dans ce flux à moins que vous ne les spécifiez explicitement ou ne les définissiez à null (pas undefined).
