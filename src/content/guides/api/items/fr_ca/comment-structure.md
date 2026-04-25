Un objet `Comment` représente un commentaire laissé par un utilisateur.

La relation entre les commentaires parent et enfant est définie via `parentId`.

La structure de l'objet `Comment` est la suivante :

[inline-code-attrs-start title = "Structure de l'objet Comment"; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Défini sur true si le moteur anti-spam a déterminé que le commentaire était du spam. **/
    aiDeterminedSpam?: boolean
    /** Indique si le commentaire est approuvé pour affichage. Défini sur true lors de l'enregistrement du commentaire, sinon il sera masqué. **/
    approved?: boolean
    /** L'avatar de l'utilisateur. **/
    avatarSrc?: string
    /** Commentaires enfants. Pas toujours peuplé dans tous les scénarios. Utilisé lorsque asTree est défini sur true via l'API. **/
    children: Comment[]
    /** Le commentaire brut de l'utilisateur. **/
    comment: string
    /** READONLY: Le commentaire de l'utilisateur analysé en HTML. **/
    commentHTML?: string
    /** L'email de l'utilisateur. Requis si les commentaires anonymes sont désactivés. **/
    commenterEmail?: string
    /** Le lien de l'utilisateur (par exemple, leur blog). **/
    commenterLink?: string
    /** Le nom de l'utilisateur. Toujours requis. Si non disponible, définir quelque chose comme "Anonyme". **/
    commenterName: string
    /** La date à laquelle le commentaire a été laissé, en epoch UTC. **/
    date: number
    /** L'étiquette d'affichage pour le commentaire - par exemple "Admin", "Moderator", ou quelque chose comme "VIP User". **/
    displayLabel?: string
    /** Le domaine sur lequel le commentaire a été posté. **/
    domain?: string
    /** READONLY: Le nombre de fois que le commentaire a été signalé. **/
    flagCount?: number
    /** Les #hashtags écrits dans le commentaire qui ont été analysés avec succès. Vous pouvez également ajouter manuellement des hashtags pour les requêtes, mais ils ne s'afficheront pas automatiquement dans le texte du commentaire. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Le commentaire contient-il des images ? **/
    hasImages?: boolean
    /** READONLY: Le commentaire contient-il des liens ? **/
    hasLinks?: boolean
    /** READONLY: L'ID unique du commentaire. **/
    id: string
    /** Seulement à la création ! Ceci est haché pour le stockage. **/
    ip?: string
    /** READONLY: L'utilisateur courant a-t-il bloqué l'auteur de ce commentaire ? **/
    isBlocked?: boolean
    /** READONLY: Le commentaire est-il de la part d'un admin ? Défini automatiquement en fonction de userId. **/
    isByAdmin?: boolean
    /** READONLY: Le commentaire est-il de la part d'un modérateur ? Défini automatiquement en fonction de userId. **/
    isByModerator?: boolean
    /** Défini sur true si le commentaire a été supprimé en douceur (un emplacement a dû être laissé en raison d'une autre configuration). **/
    isDeleted?: boolean
    /** Défini sur true si le compte de l'utilisateur a été supprimé et que le commentaire a dû être conservé. **/
    isDeletedUser?: boolean
    /** READONLY: Est-ce que le commentaire a été signalé par l'utilisateur actuellement connecté (contextUserId) ? **/
    isFlagged?: boolean
    /** Le commentaire est-il épinglé ? **/
    isPinned?: boolean
    /** Le commentaire est-il verrouillé ? Lorsqu'il est true, personne (y compris les modérateurs) ne peut y répondre, le modifier ou le supprimer jusqu'à ce qu'il soit déverrouillé. **/
    isLocked?: boolean
    /** Le commentaire est-il du spam ? **/
    isSpam?: boolean
    /** READONLY: Le commentaire est-il voté négativement pour l'utilisateur courant (contextUserId) ? **/
    isVotedDown?: boolean
    /** READONLY: Le commentaire est-il voté positivement pour l'utilisateur courant (contextUserId) ? **/
    isVotedUp?: boolean
    /** La locale du commentaire. Si non fournie, elle sera dérivée de l'en-tête HTTP Accept-Language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: Les @mentions écrites dans le commentaire qui ont été analysées avec succès. **/
    mentions?: CommentUserMention[]
    /** Métadonnées optionnelles associées au commentaire. **/
    meta?: Record<string, string | number | boolean>
    /** La liste optionnelle des ids de groupes de modération associés à ce commentaire. **/
    moderationGroupIds?: string[]|null
    /** READONLY: L'id de l'objet de vote correspondant au vote de l'utilisateur courant (contextUserId) sur ce commentaire. **/
    myVoteId?: string
    /** Indique si des notifications ont été envoyées pour ce commentaire aux commentateurs. Pour éviter l'envoi de notifications lors d'importations, définissez ceci sur true. **/
    notificationSentForParent?: boolean
    /** Indique si des notifications ont été envoyées pour ce commentaire aux utilisateurs du tenant. Pour éviter l'envoi de notifications lors d'importations, définissez ceci sur true. **/
    notificationSentForParentTenant?: boolean
    /** Le titre de la page sur laquelle se trouvait ce commentaire. **/
    pageTitle?: string
    /** Si nous répondons à un commentaire, c'est l'ID auquel nous répondons. **/
    parentId?: string|null
    /** Indique si le commentaire est marqué comme revu. **/
    reviewed: boolean
    /** L'id du tenant auquel appartient le commentaire. **/
    tenantId: string
    /** L'utilisateur qui a écrit le commentaire. Créé automatiquement lors de l'enregistrement d'un commentaire avec un nom/email. **/
    userId?: string|null
    /** L'URL de l'emplacement où ce commentaire est visible, comme un billet de blog. **/
    url: string
    /** Une version "nettoyée" du urlId que vous nous avez fourni. Lors de l'enregistrement, vous spécifiez ce champ, mais lorsque vous récupérez le commentaire, il sera "nettoyé" et votre valeur originale déplacée vers "urlIdRaw". **/
    urlId: string
    /** READONLY: Le urlId original que vous nous avez fourni. **/
    urlIdRaw?: string
    /** L'utilisateur et ce commentaire sont-ils vérifiés ? **/
    verified: boolean
    /** Nombre de votes positifs. **/
    votesUp?: number
    /** Nombre de votes négatifs. **/
    votesDown?: number
    /** Le "karma" du commentaire (= votes positifs - votes négatifs). **/
    votes?: number
}
[inline-code-end]

Certains de ces champs sont marqués `READONLY` - ils sont renvoyés par l'API mais ne peuvent pas être définis.

### Structure du texte du commentaire

Les commentaires sont rédigés dans une variante FastComments du markdown, qui est simplement du markdown avec des balises de style `bbcode` traditionnelles pour les images, comme `[img]path[/img]`.

Le texte est stocké dans deux champs. Le texte saisi par l'utilisateur est stocké sans modification dans le champ `comment`. Celui-ci est rendu et stocké dans le champ `commentHTML`.

Les balises HTML autorisées sont `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Il est recommandé de rendre le HTML, puisqu'il s'agit d'un très petit sous-ensemble de HTML, la création d'un moteur de rendu est assez simple. Il existe plusieurs bibliothèques pour React Native et Flutter, par exemple, pour aider à cela

Vous pouvez choisir de rendre la valeur non normalisée du champ `comment`. [Un exemple de parseur est ici.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

L'exemple de parseur peut aussi être ajusté pour fonctionner avec du HTML, et transformer les balises HTML en éléments attendus à rendre pour votre plateforme. 

### Mentions

Lorsque des utilisateurs sont mentionnés dans un commentaire, l'information est stockée dans une liste appelée `mentions`. Chaque objet de cette liste a la structure suivante.

[inline-code-attrs-start title = "Structure de l'objet CommentUserMention"; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'id de l'utilisateur. Pour les utilisateurs SSO, cela aura votre tenant id préfixé. **/
    id: string
    /** Le texte final de la balise @mention, incluant le symbole @. **/
    tag: string
    /** Le texte original de la balise @mention, incluant le symbole @. **/
    rawTag: string
    /** Le type d'utilisateur qui a été mentionné. user = compte FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si l'utilisateur se désinscrit des notifications, ceci sera quand même défini sur true. **/
    sent: boolean
}
[inline-code-end]

### Hashtags

Lorsque des hashtags sont utilisés et analysés avec succès, l'information est stockée dans une liste appelée `hashTags`. Chaque objet de cette liste a la structure suivante. Les hashtags peuvent aussi être ajoutés manuellement au tableau `hashTags` du commentaire pour les requêtes, si `retain` est défini.

[inline-code-attrs-start title = "Structure de l'objet CommentHashTag"; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** L'id du hashtag. **/
    id: string
    /** Le texte final de la balise #hashtag, incluant le symbole #. **/
    tag: string
    /** Si le hashtag est associé à une URL personnalisée, ceci sera défini. **/
    url?: string
    /** Si nous devons conserver le hashtag, même s'il n'existe pas dans le texte du commentaire, lorsque le commentaire est mis à jour. Utile pour taguer des commentaires sans modifier le texte du commentaire. **/
    retain?: boolean
}
[inline-code-end]

---