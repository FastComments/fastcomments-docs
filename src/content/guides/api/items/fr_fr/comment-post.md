[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de créer des commentaires.

Les cas d'utilisation courants sont les interfaces personnalisées, les intégrations ou les importations.

Notes :

- Cette API peut mettre à jour le widget de commentaires "en direct" si désiré (cela augmente le `creditsCost` de `1` à `2`).
- Cette API créera automatiquement des objets utilisateur dans notre système si un email est fourni.
- Essayer d'enregistrer deux commentaires avec des emails différents, mais le même nom d'utilisateur, entraînera une erreur pour le deuxième commentaire.
- Si vous spécifiez `parentId`, et qu'un commentaire enfant a `notificationSentForParent` à false, **nous enverrons des notifications pour le commentaire parent**. Cela se fait toutes les heures (nous regroupons les notifications pour diminuer le nombre d'emails envoyés).
- Si vous voulez envoyer des emails de bienvenue lors de la création d'utilisateurs, ou des emails de vérification de commentaires, définissez `sendEmails` à `true` dans les paramètres de requête.
- Les commentaires créés via cette API apparaîtront dans les pages Analytique et Modération de l'application admin.
- Les "mauvais mots" sont toujours masqués dans les noms des commentateurs et le texte des commentaires si le paramètre est activé.
- Les commentaires créés via cette API peuvent toujours être vérifiés pour le spam si désiré.
- Les configurations telles que la longueur maximale des commentaires, si configurées via la page d'administration des règles de personnalisation, s'appliqueront ici.

Les données minimales requises pour soumettre qui s'afficheront dans le widget de commentaires sont les suivantes :

[inline-code-attrs-start title = 'Exemple cURL Minimum de POST de Commentaire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Une requête plus réaliste pourrait ressembler à :

[inline-code-attrs-start title = 'Exemple cURL de POST de Commentaire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête POST de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse POST de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Included on failure. **/
    reason?: string
    /** The created comment. **/
    comment?: Comment
    /** The associated user, which may or may not have already existed. **/
    user?: User
}
[inline-code-end]
