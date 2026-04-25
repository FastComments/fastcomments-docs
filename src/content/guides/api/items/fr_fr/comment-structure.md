---
Un objet `Comment` représente un commentaire laissé par un utilisateur.

La relation entre les commentaires parent et enfant est définie via `parentId`.

La structure de l'objet Comment est la suivante :

[inline-code-attrs-start title = 'Structure de l\'objet Comment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** EN LECTURE SEULE : Défini sur true si le moteur anti-spam a déterminé que le commentaire était du spam. **/
    aiDeterminedSpam?: boolean
    /** Indique si le commentaire est approuvé pour être affiché. Défini sur true lors de l'enregistrement du commentaire, sinon il sera masqué. **/
    approved?: boolean
    /** L'avatar de l'utilisateur. **/
    avatarSrc?: string
    /** Commentaires enfants. Non renseignés dans tous les scénarios. Utilisé lorsque asTree est défini sur true via l'API. **/
    children: Comment[]
    /** Le commentaire brut de l'auteur. **/
    comment: string
    /** EN LECTURE SEULE : Le commentaire de l'auteur analysé en HTML. **/
    commentHTML?: string
    /** L'email de l'auteur du commentaire. Requis si les commentaires anonymes sont désactivés. **/
    commenterEmail?: string
    /** Le lien de l'auteur (par exemple, leur blog). **/
    commenterLink?: string
    /** Le nom de l'auteur. Toujours requis. Si non disponible, définissez quelque chose comme "Anonyme". **/
    commenterName: string
    /** La date à laquelle le commentaire a été laissé, en epoch UTC. **/
    date: number
    /** Le "label d'affichage" du commentaire - par exemple "Admin", "Modérateur", ou quelque chose comme "Utilisateur VIP". **/
    displayLabel?: string
    /** Le domaine sur lequel le commentaire a été publié. **/
    domain?: string
    /** EN LECTURE SEULE : Le nombre de fois que le commentaire a été signalé. **/
    flagCount?: number
    /** Les #hashtags écrits dans le commentaire qui ont été correctement analysés. Vous pouvez également ajouter des hashtags manuellement, pour les requêtes, mais ils ne s'afficheront pas automatiquement dans le texte du commentaire. **/
    hashTags?: CommentHashTag[]
    /** EN LECTURE SEULE : Le commentaire contient-il des images ? **/
    hasImages?: boolean
    /** EN LECTURE SEULE : Le commentaire contient-il des liens ? **/
    hasLinks?: boolean
    /** EN LECTURE SEULE : L'id unique du commentaire. **/
    id: string
    /** Seulement à la création ! Ceci est haché pour le stockage. **/
    ip?: string
    /** EN LECTURE SEULE : L'utilisateur courant a-t-il bloqué l'utilisateur qui a écrit ce commentaire ? **/
    isBlocked?: boolean
    /** EN LECTURE SEULE : Le commentaire est-il écrit par un administrateur ? Défini automatiquement en fonction de userId. **/
    isByAdmin?: boolean
    /** EN LECTURE SEULE : Le commentaire est-il écrit par un modérateur ? Défini automatiquement en fonction de userId. **/
    isByModerator?: boolean
    /** Défini sur true si le commentaire a été supprimé en douceur (un emplacement de remplacement a dû être conservé en raison d'une autre configuration). **/
    isDeleted?: boolean
    /** Défini sur true si le compte de l'utilisateur a été supprimé et que le commentaire a dû être conservé. **/
    isDeletedUser?: boolean
    /** EN LECTURE SEULE : Est-ce que le commentaire a été signalé par l'utilisateur actuellement connecté (contextUserId) ? **/
    isFlagged?: boolean
    /** Le commentaire est-il épinglé ? **/
    isPinned?: boolean
    /** Le commentaire est-il verrouillé ? Quand true, personne (y compris les modérateurs) ne peut y répondre, le modifier ou le supprimer tant qu'il n'est pas déverrouillé. **/
    isLocked?: boolean
    /** Le commentaire est-il du spam ? **/
    isSpam?: boolean
    /** EN LECTURE SEULE : Le commentaire a-t-il été voté négativement par l'utilisateur courant (contextUserId) ? **/
    isVotedDown?: boolean
    /** EN LECTURE SEULE : Le commentaire a-t-il été voté positivement par l'utilisateur courant (contextUserId) ? **/
    isVotedUp?: boolean
    /** La locale du commentaire. Si non fournie, elle sera dérivée de l'en-tête HTTP Accept-Language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** EN LECTURE SEULE : Les @mentions écrites dans le commentaire qui ont été correctement analysées. **/
    mentions?: CommentUserMention[]
    /** Métadonnées optionnelles associées au commentaire. **/
    meta?: Record<string, string | number | boolean>
    /** La liste optionnelle des ids de groupe de modération associés à ce commentaire. **/
    moderationGroupIds?: string[]|null
    /** EN LECTURE SEULE : L'id de l'objet de vote qui correspond au vote de l'utilisateur courant (contextUserId) sur ce commentaire. **/
    myVoteId?: string
    /** Indique si des notifications ont été envoyées pour ce commentaire aux commentateurs. Pour empêcher l'envoi de notifications lors des importations, définissez ceci sur true. **/
    notificationSentForParent?: boolean
    /** Indique si des notifications ont été envoyées pour ce commentaire aux utilisateurs du tenant. Pour empêcher l'envoi de notifications lors des importations, définissez ceci sur true. **/
    notificationSentForParentTenant?: boolean
    /** Le titre de la page sur laquelle portait ce commentaire. **/
    pageTitle?: string
    /** Si nous répondons à un commentaire, voici l'ID auquel nous répondons. **/
    parentId?: string|null
    /** Indique si le commentaire est marqué comme revu. **/
    reviewed: boolean
    /** L'id du tenant auquel appartient le commentaire. **/
    tenantId: string
    /** L'utilisateur qui a écrit le commentaire. Créé automatiquement lors de l'enregistrement d'un commentaire avec un nom/email. **/
    userId?: string|null
    /** L'URL de l'emplacement où ce commentaire est visible, comme un article de blog. **/
    url: string
    /** Une version "nettoyée" de l'urlId que vous nous avez fourni. Lors de l'enregistrement, vous spécifiez ce champ, mais lorsque vous récupérez le commentaire, celui-ci sera "nettoyé" et votre valeur originale déplacée vers "urlIdRaw". **/
    urlId: string
    /** EN LECTURE SEULE : L'urlId original que vous nous avez fourni. **/
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

Certaines de ces propriétés sont marquées `READONLY` - elles sont renvoyées par l'API mais ne peuvent pas être définies.

### Structure du texte du commentaire

Les commentaires sont écrits dans une variante FastComments du markdown, qui est simplement du markdown plus des balises de style `bbcode` traditionnelles pour les images, comme `[img]path[/img]`.

Le texte est stocké dans deux champs. Le texte saisi par l'utilisateur est stocké non modifié dans le champ `comment`. Il est rendu et stocké dans le champ `commentHTML`.

Les balises HTML autorisées sont `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Il est recommandé de rendre le HTML, puisqu'il s'agit d'un sous-ensemble très réduit de HTML, la création d'un moteur de rendu est assez simple. Il existe, par exemple, plusieurs bibliothèques pour React Native et Flutter pour vous aider.

Vous pouvez choisir de rendre la valeur non normalisée du champ `comment`. [Un exemple de parser est ici.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

L'exemple de parser peut également être adapté pour fonctionner avec du HTML, et transformer les balises HTML en éléments attendus à rendre pour votre plateforme. 

### Tagging

Lorsque des utilisateurs sont mentionnés dans un commentaire, l'information est stockée dans une liste appelée `mentions`. Chaque objet de cette liste
a la structure suivante.

[inline-code-attrs-start title = 'Objet des mentions du commentaire'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'id de l'utilisateur. Pour les utilisateurs SSO, cela aura votre id de tenant en préfixe. **/
    id: string
    /** Le texte final de la balise @mention, incluant le symbole @. **/
    tag: string
    /** Le texte original de la balise @mention, incluant le symbole @. **/
    rawTag: string
    /** Quel type d'utilisateur a été mentionné. user = compte FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si l'utilisateur choisit de ne pas recevoir de notifications, ceci sera tout de même défini sur true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Lorsque des hashtags sont utilisés et correctement analysés, les informations sont stockées dans une liste appelée `hashTags`. Chaque objet de cette liste
a la structure suivante. Les hashtags peuvent également être ajoutés manuellement au tableau `hashTags` du commentaire pour les requêtes, si `retain` est défini.

[inline-code-attrs-start title = 'Objet HashTag du commentaire'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** L'id du hashtag. **/
    id: string
    /** Le texte final de la balise #hashtag, incluant le symbole #. **/
    tag: string
    /** Si le hashtag est associé à une URL personnalisée, celle-ci sera définie. **/
    url?: string
    /** Si nous devons conserver le hashtag, même s'il n'existe pas dans le texte du commentaire, lorsque le commentaire est mis à jour. Utile pour taguer des commentaires sans modifier le texte du commentaire. **/
    retain?: boolean
}
[inline-code-end]

---