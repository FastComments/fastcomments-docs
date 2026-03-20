Un objet `Comment` représente un commentaire laissé par un utilisateur.

La relation entre les commentaires parents et enfants est définie via `parentId`.

La structure de l'objet Comment est la suivante :

[inline-code-attrs-start title = 'Structure de Comment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Set to true if the spam engine determined the comment was spam. **/
    aiDeterminedSpam?: boolean
    /** Whether the comment is approved to show. Set to true when saving the comment, else it will be hidden. **/
    approved?: boolean
    /** The user's avatar. **/
    avatarSrc?: string
    /** Child comments. Not populated in all scenarios. Used when asTree is set to true via the API. **/
    children: Comment[]
    /** The commenter's raw comment. **/
    comment: string
    /** READONLY: The commenter's comment parsed into HTML. **/
    commentHTML?: string
    /** The commenter's email. Required if anonymous commenting is off. **/
    commenterEmail?: string
    /** The commenter's link (for example, their blog). **/
    commenterLink?: string
    /** The commenter's name. Always required. If not available, set to something like "Anonymous". **/
    commenterName: string
    /** The date the comment was left, in UTC epoch. **/
    date: number
    /** The "display label" for the comment - for example "Admin", "Moderator", or something like "VIP User". **/
    displayLabel?: string
    /** The domain the comment was posted on. **/
    domain?: string
    /** READONLY: The number of times the comment was flagged. **/
    flagCount?: number
    /** The #hashtags written in the comment that were successfully parsed. You can also manually add hashtags, for querying, but they won't display in the comment text automatically. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Does the comment contain images? **/
    hasImages?: boolean
    /** READONLY: Does the comment contain links? **/
    hasLinks?: boolean
    /** READONLY: The unique comment id. **/
    id: string
    /** Only on create! This is hashed for storage. **/
    ip?: string
    /** READONLY: Did the current user block the user that wrote this comment? **/
    isBlocked?: boolean
    /** READONLY: Is the comment by an admin? Automatically set based on userId. **/
    isByAdmin?: boolean
    /** READONLY: Is the comment by a moderator? Automatically set based on userId. **/
    isByModerator?: boolean
    /** Set to true if the comment was soft deleted (placeholder had to be left due to some other configuration). **/
    isDeleted?: boolean
    /** Set to true if the user's account was deleted and the comment had to be retained. **/
    isDeletedUser?: boolean
    /** READONLY: Is the flagged by the currently logged-in user (contextUserId)? **/
    isFlagged?: boolean
    /** Is the comment pinned? **/
    isPinned?: boolean
    /** Is the comment locked for new replies (moderators still can reply)? **/
    isLocked?: boolean
    /** Is the comment spam? **/
    isSpam?: boolean
    /** READONLY: Is the comment voted down for the current user (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: Is the comment voted up for the current user (contextUserId)? **/
    isVotedUp?: boolean
    /** The locale the comment is in. If not provided, will be derived from the language accept HTTP header. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: The @mentions written in the comment that were successfully parsed. **/
    mentions?: CommentUserMention[]
    /** Optional metadata associated with the comment. **/
    meta?: Record<string, string | number | boolean>
    /** The optional list of moderation group ids associated with this comment. **/
    moderationGroupIds?: string[]|null
    /** READONLY: The id of the vote object that corresponds to the vote from the current user (contextUserId) on this comment. **/
    myVoteId?: string
    /** Whether notifications were sent for this comment for commenters. To prevent notifications being sent on imports, set this to true. **/
    notificationSentForParent?: boolean
    /** Whether notifications were sent for this comment for tenant users. To prevent notifications being sent on imports, set this to true. **/
    notificationSentForParentTenant?: boolean
    /** The title of the page this comment was on. **/
    pageTitle?: string
    /** If we're replying to a comment, this is the ID that we are replying to. **/
    parentId?: string|null
    /** Whether the comment is marked reviewed. **/
    reviewed: boolean
    /** The tenant id where the comment belongs. **/
    tenantId: string
    /** The user that wrote the comment. Created automatically when saving a comment with a name/email. **/
    userId?: string|null
    /** The URL to the location that this comment is visible, like a blog post. **/
    url: string
    /** A "cleaned" version of the urlId you passed us. When saving, you specify this field, but when you fetch the comment back this will be "cleaned" and your original value moved to "urlIdRaw". **/
    urlId: string
    /** READONLY: The original urlId you passed us. **/
    urlIdRaw?: string
    /** Is the user and this comment verified? **/
    verified: boolean
    /** Number of votes up. **/
    votesUp?: number
    /** Number of votes down. **/
    votesDown?: number
    /** The "karma" of the comment (= votes up - votes down). **/
    votes?: number
}
[inline-code-end]

Certains de ces champs sont marqués `READONLY` - ils sont retournés par l'API mais ne peuvent pas être définis.

### Structure du texte de commentaire

Les commentaires sont écrits dans une version FastComments de markdown, qui est simplement markdown plus des balises de style `bbcode` traditionnelles pour les images, comme `[img]path[/img]`.

Le texte est stocké dans deux champs. Le texte entré par l'utilisateur est stocké non modifié dans le champ `comment`. Il est rendu et stocké dans le champ `commentHTML`.

Les balises HTML autorisées sont `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, et br`.

Il est recommandé de rendre le HTML, car c'est un sous-ensemble très petit de HTML, construire un moteur de rendu est assez simple. Il existe plusieurs bibliothèques pour React Native et Flutter, par exemple, pour aider avec cela.

Vous pouvez choisir de rendre la valeur non normalisée du champ `comment`. [Un exemple d'analyseur est ici.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

L'exemple d'analyseur pourrait également être ajusté pour fonctionner avec HTML, et transformer les balises HTML en éléments attendus à rendre pour votre plateforme.

### Marquage

Lorsque des utilisateurs sont tagués dans un commentaire, l'information est stockée dans une liste appelée `mentions`. Chaque objet dans cette liste
a la structure suivante.

[inline-code-attrs-start title = 'L\'objet Mentions de Commentaire'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Lorsque des hashtags sont utilisés et analysés avec succès, l'information est stockée dans une liste appelée `hashTags`. Chaque objet dans cette liste
a la structure suivante. Les hashtags peuvent également être ajoutés manuellement au tableau `hashTags` du commentaire pour la requête, si `retain` est défini.

[inline-code-attrs-start title = 'L\'objet HashTag de Commentaire'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** The hashtag id. **/
    id: string
    /** The final #hashtag tag text, including the # symbol. **/
    tag: string
    /** If the hashtag is associated with a custom URL, this will be defined. **/
    url?: string
    /** If we should retain the hashtag, even if it does not exist in the comment text, when the comment is updated. Useful for tagging comments without changing comment text. **/
    retain?: boolean
}
[inline-code-end]
