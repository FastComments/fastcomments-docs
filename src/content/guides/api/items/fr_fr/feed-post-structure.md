Un objet `FeedPost` représente un article dans un flux FastComments. Un flux est un flux d'articles avec leurs propres fils de commentaires, rendu par le widget Feed. Chaque article possède un auteur, un contenu riche optionnel, des médias et des liens, et peut être étiqueté afin qu'un flux puisse être filtré.

La structure de l'objet `FeedPost` est la suivante :

[inline-code-attrs-start title = 'Structure FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** L'identifiant de l'utilisateur FastComments ou SSO qui a créé l'article. **/
    fromUserId?: string
    /** Rempli à partir de l'utilisateur lorsqu'il n'est pas défini. **/
    fromUserDisplayName?: string | null
    /** READONLY. Rempli à partir de l'utilisateur. **/
    fromUserAvatar?: string | null
    /** Utilisé pour filtrer un flux. **/
    tags?: string[]
    /** Poids de tri dans un flux. Des valeurs plus élevées sont triées en premier. **/
    weight?: number
    /** Paires clé/valeur libres pour votre propre usage. **/
    meta?: Record<string, string>
    /** HTML désinfecté. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Type de réaction à compter. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** L'URL vers lequel l'élément média pointe lorsqu'on clique dessus. **/
    linkUrl?: string
    /** Une entrée par rendu. Le widget choisit le meilleur ajustement. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Le texte du lien, comme « Inscrivez‑vous maintenant ». **/
    text?: string
    /** Un titre affiché avec le lien. **/
    title?: string
    /** Une description affichée avec le lien. **/
    description?: string
    url?: string
}
[inline-code-end]

Remarques :

- Certains de ces champs sont marqués `READONLY` – ils sont renvoyés par l'API mais ne peuvent pas être définis.
- Les commentaires sur un article sont des commentaires ordinaires dont le `urlId` est `post:` suivi de l'`_id` de l'article. Utilisez cette valeur avec l'API Comment pour lire ou créer des commentaires sur un article.