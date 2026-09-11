The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### Structure de l'objet WebhookComment

##### Structure de l'événement "Create"
The "create" event request body is a WebhookComment object.

##### Structure de l'événement "Update"
The "update" event request body is a WebhookComment object.

##### Structure de l'événement "Delete"
The "delete" event request body is a WebhookComment object.

    Modification à partir du 14 nov. 2023
    Auparavant, le corps de la requête de l'événement "delete" ne contenait que l'ID du commentaire. Il contient maintenant le commentaire complet au moment de la suppression.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

Chaque clé est toujours présente dans le corps. Lorsqu'un commentaire n'a pas de valeur pour un champ, le corps contient `null` (ou `false` pour les booléens et `[]` pour les listes), de sorte que la forme d'une livraison ne varie jamais d'un commentaire à l'autre.

[inline-code-attrs-start title = 'L\'objet WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'ID du commentaire. **/
    id: string
    /** L'ID ou l'URL qui identifie le fil de commentaires. Normalisé. **/
    urlId: string
    /** L'URL qui pointe vers l'endroit où le commentaire a été laissé. **/
    url: string | null
    /** L'ID utilisateur qui a laissé le commentaire. Si SSO, préfixé avec l'ID du locataire. **/
    userId: string | null
    /** L'email de l'utilisateur qui a laissé le commentaire. **/
    commenterEmail: string | null
    /** Le nom de l'utilisateur affiché dans le widget de commentaire. Avec SSO, peut être displayName. **/
    commenterName: string
    /** Texte brut du commentaire. **/
    comment: string
    /** Texte du commentaire après analyse. **/
    commentHTML: string
    /** ID externe du commentaire. **/
    externalId: string | null
    /** L'ID du commentaire parent. **/
    parentId: string | null
    /** La date UTC à laquelle le commentaire a été laissé. **/
    date: UTC_ISO_DateString
    /** Karma combiné (up - down) des votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Vrai si l'utilisateur était connecté lorsqu'il a commenté, ou s'il a vérifié le commentaire, ou s'il a vérifié sa session lorsque le commentaire a été laissé. **/
    verified: boolean
    /** La date UTC à laquelle le commentaire a été vérifié. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Si un modérateur a marqué le commentaire comme revu. **/
    reviewed: boolean
    /** L'emplacement, ou l'encodage base64, de l'avatar. Sera uniquement base64 si c'était la valeur transmise avec SSO. **/
    avatarSrc: string | null
    /** Le commentaire a-t-il été marqué manuellement ou automatiquement comme spam ? **/
    isSpam: boolean
    /** Le commentaire a-t-il été marqué automatiquement comme spam ? **/
    aiDeterminedSpam: boolean
    /** Y a-t-il des images dans le commentaire ? **/
    hasImages: boolean
    /** Le numéro de page du commentaire pour le tri « Most Relevant ». **/
    pageNumber: number | null
    /** Le numéro de page du commentaire pour le tri « Oldest First ». **/
    pageNumberOF: number | null
    /** Le numéro de page du commentaire pour le tri « Newest First ». **/
    pageNumberNF: number | null
    /** Le commentaire a-t-il été approuvé automatiquement ou manuellement ? **/
    approved: boolean
    /** Le code de locale (format : en_us) de l'utilisateur lorsque le commentaire a été rédigé. **/
    locale: string | null
    /** Les @mentions écrites dans le commentaire qui ont été analysées avec succès. Vide lorsqu'il n'y en a aucune. **/
    mentions: CommentUserMention[]
    /** Le domaine d'où provient le commentaire. **/
    domain: string | null
    /** Les IDs des groupes de modération associés à ce commentaire. Vide lorsqu'il n'y en a aucun. **/
    moderationGroupIds: string[]
}
[inline-code-end]

Lorsque des utilisateurs sont mentionnés dans un commentaire, l'information est stockée dans une liste appelée `mentions`. Chaque objet de cette liste a la structure suivante.

[inline-code-attrs-start title = 'L\'objet Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'ID utilisateur. Pour les utilisateurs SSO, il sera préfixé avec votre ID de locataire. **/
    id: string
    /** Le texte final de la balise @mention, incluant le symbole @. **/
    tag: string
    /** Le texte original de la balise @mention, incluant le symbole @. **/
    rawTag: string
    /** Quel type d'utilisateur a été mentionné. user = compte FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si l'utilisateur se désinscrit des notifications, cela restera à true. **/
    sent: boolean
}
[inline-code-end]

#### Méthodes HTTP

Vous pouvez configurer la méthode HTTP pour chaque type d'événement webhook dans le panneau d'administration :

- **Événement de création** : POST ou PUT (par défaut : PUT)
- **Événement de mise à jour** : POST ou PUT (par défaut : PUT)
- **Événement de suppression** : DELETE, POST ou PUT (par défaut : DELETE)

Comme toutes les requêtes contiennent un ID, les opérations de création et de mise à jour sont idempotentes par défaut (PUT). Répéter la même requête de création ou de mise à jour ne doit pas créer d'objets en double de votre côté.

#### En-têtes de requête

Chaque requête webhook inclut les en-têtes suivants :

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Votre secret d'API |
| `X-FastComments-Timestamp` | Horodatage Unix (secondes) lorsque la requête a été signée |
| `X-FastComments-Signature` | Signature HMAC‑SHA256 (`sha256=<hex>`) |

Voir [Sécurité et jetons d'API](/guide-webhooks.html#webhooks-api-tokens) pour plus d'informations sur la vérification de la signature HMAC.