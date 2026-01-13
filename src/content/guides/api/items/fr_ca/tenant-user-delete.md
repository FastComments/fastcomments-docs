[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Cette route permet la suppression d'un `TenantUser` par id.

La suppression des commentaires de l'utilisateur est possible via le paramètre de requête `deleteComments`. Notez que si cela est vrai :

1. Tous les commentaires de l'utilisateur seront supprimés en direct.
2. Tous les commentaires __enfants__ (maintenant orphelins) seront supprimés ou anonymisés selon la configuration de page associée à chaque commentaire. Par exemple, si le mode de suppression de fil est "anonymiser", alors les réponses resteront, et les commentaires de l'utilisateur seront anonymisés. Cela s'applique uniquement lorsque `commentDeleteMode` est `Remove` (la valeur par défaut).
3. Le `creditsCost` devient `2`.

### Commentaires Anonymisés

Vous pouvez conserver les commentaires de l'utilisateur mais simplement les anonymiser en définissant `commentDeleteMode=1`.

Si les commentaires de l'utilisateur sont anonymisés, les valeurs suivantes sont définies à null :

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` et `isDeletedUser` sont définis à `true`.

Lors du rendu, le widget de commentaires utilisera `DELETED_USER_PLACEHOLDER` (par défaut : "[deleted]") pour le nom de l'utilisateur et `DELETED_CONTENT_PLACEHOLDER` pour le commentaire. Ceux-ci peuvent être personnalisés via l'interface de personnalisation du widget.

### Exemples

[inline-code-attrs-start title = 'Exemple cURL de Suppression de TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Suppression de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Suppression de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
