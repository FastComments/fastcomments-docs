[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de mettre à jour un commentaire unique.

Notes :

- Cette API peut mettre à jour le widget de commentaires "en direct" si désiré (cela augmente le `creditsCost` de base de `1` à `2`).
  - Cela peut rendre la migration des commentaires entre les pages "en direct" (changement de `urlId`).
  - Les migrations coûtent `2` crédits supplémentaires car les pages sont précalculées et cela est intensif en CPU.
- Contrairement à l'API de création, cette API ne créera PAS automatiquement des objets utilisateur dans notre système si un email est fourni.
- Les commentaires mis à jour via cette API peuvent toujours être vérifiés pour le spam si désiré.
- Les configurations telles que la longueur maximale des commentaires, si configurées via la page d'administration des règles de personnalisation, s'appliqueront ici.
- Pour permettre aux utilisateurs de mettre à jour leur texte de commentaire, vous pouvez simplement spécifier `comment` dans le corps de la requête. Nous générerons le `commentHTML` résultant.
  - Si vous définissez à la fois `comment` et `commentHTML`, nous ne générerons pas automatiquement le HTML.
  - Si l'utilisateur ajoute des mentions ou des hashtags dans son nouveau texte, il sera toujours traité comme l'API `POST`.
- Lors de la mise à jour de `commenterEmail` sur un commentaire, il est préférable de spécifier également `userId`. Sinon, vous devez vous assurer que l'utilisateur avec cet email appartient à votre locataire, sinon la requête échouera.


[inline-code-attrs-start title = 'Exemple cURL Minimum de PATCH de Commentaire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête PATCH de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can edit the comment.  **/
    contextUserId?: string
	/** Should we check if the new comment looks like spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse PATCH de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
