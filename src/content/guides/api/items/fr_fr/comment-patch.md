[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ce point de terminaison de l'API permet de mettre à jour un seul commentaire.

Remarques :

- Cette API peut mettre à jour le widget de commentaires "live" si souhaité (cela augmente le `creditsCost` de base de `1` à `2`).
  - Cela peut permettre de migrer des commentaires entre des pages "en direct" (en changeant `urlId`).
  - Les migrations coûtent `2` crédits supplémentaires car les pages sont précalculées et cela est intensif en CPU.
- Contrairement à l'API de création, cette API NE créera PAS automatiquement d'objets utilisateur dans notre système si un email est fourni.
- Les commentaires mis à jour via cette API peuvent toujours être vérifiés pour spam si nécessaire.
- La configuration telle que la longueur maximale des commentaires, si configurée via la page d'administration des règles de personnalisation (Customization Rule), s'appliquera ici.
- Pour permettre aux utilisateurs de mettre à jour le texte de leur commentaire, vous pouvez simplement spécifier `comment` dans le corps de la requête. Nous générerons le `commentHTML` résultant.
  - Si vous définissez à la fois `comment` et `commentHTML`, nous ne générerons pas automatiquement le HTML.
  - Si l'utilisateur ajoute des mentions ou des hashtags dans son nouveau texte, ils seront toujours traités comme pour l'API `POST`.
- Lors de la mise à jour de `commenterEmail` sur un commentaire, il est préférable de spécifier également `userId`. Sinon, vous devez vous assurer que l'utilisateur avec cet email appartient à votre tenant, sinon la requête échouera.  
- Si le commentaire cible est verrouillé (`isLocked: true`), la requête est rejetée avec `code: 'locked'`. Déverrouillez d'abord le commentaire, mettez-le à jour, puis reverrouillez si souhaité.


[inline-code-attrs-start title = 'Exemple cURL minimal pour PATCH de commentaire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête PATCH de commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** L'utilisateur effectuant la mise à jour. Si souhaité, peut être utilisé pour vérifier qu'il peut modifier le commentaire.  **/
    contextUserId?: string
	/** Doit-on vérifier si le nouveau commentaire ressemble à du spam ?  **/
    doSpamCheck?: 'true' | 'false'
	/** Si le commentaire doit apparaître "live" pour les utilisateurs visualisant des instances du widget de commentaires avec le même urlId. REMARQUE : double le coût en crédits de 1 à 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse PATCH de commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Inclus en cas d'échec. **/
    reason?: string
}
[inline-code-end]