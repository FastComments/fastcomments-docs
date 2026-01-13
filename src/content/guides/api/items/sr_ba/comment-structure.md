Objekat `Comment` predstavlja komentar koji je ostavio korisnik.

Veza između roditeljskih i podkomentara definiše se preko `parentId`.

Struktura objekta Comment je sljedeća:

[inline-code-attrs-start title = 'Struktura komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Postavljeno na true ako spam mehanizam odredi da je komentar spam. **/
    aiDeterminedSpam?: boolean
    /** Da li je komentar odobren za prikaz. Postavi se na true prilikom čuvanja komentara, inače će biti skriven. **/
    approved?: boolean
    /** Avatar korisnika. **/
    avatarSrc?: string
    /** Podkomentari. Nisu popunjeni u svim scenarijima. Koristi se kada je asTree postavljeno na true preko API-ja. **/
    children: Comment[]
    /** Neobrađeni komentar koji je ostavio korisnik. **/
    comment: string
    /** READONLY: Komentar korisnika parsiran u HTML. **/
    commentHTML?: string
    /** Email komentatora. Obavezan ako je anonimno komentarisanje isključeno. **/
    commenterEmail?: string
    /** Link komentatora (npr. njihov blog). **/
    commenterLink?: string
    /** Ime komentatora. Uvijek obavezno. Ako nije dostupno, postavite npr. "Anonymous". **/
    commenterName: string
    /** Datum ostavljanja komentara, u UTC epoch formatu. **/
    date: number
    /** 'Prikazna oznaka' za komentar – npr. "Admin", "Moderator" ili nešto poput "VIP User". **/
    displayLabel?: string
    /** Domen na kojem je komentar objavljen. **/
    domain?: string
    /** READONLY: Broj puta koliko je komentar prijavljen/označen. **/
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

Neka od ovih polja su označena kao `READONLY` - ta polja vraća API, ali ih nije moguće postaviti.

### Comment Text Structure

Komentari se pišu u FastComments varijanti markdown-a, što je u suštini markdown plus tradicionalne `bbcode` oznake za slike, poput `[img]path[/img]`.

Tekst se čuva u dva polja. Tekst koji je korisnik unio čuva se neizmijenjen u polju `comment`. Ovaj tekst se renduje i sprema u polje `commentHTML`.

Dozvoljene HTML oznake su `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Preporučuje se renderovanje HTML-a, pošto je to vrlo mali podskup HTML-a i izgradnja renderer-a je prilično jednostavna. Postoji više biblioteka za React Native i Flutter, na primjer, koje u tome pomažu

Možete izabrati da renderujete nenormalizovanu vrijednost polja `comment`. [An example parser is here.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Primjer parsera se također može prilagoditi da radi sa HTML-om i transformiše HTML oznake u očekivane elemente za renderovanje na vašoj platformi. 

### Tagging

Kada su korisnici označeni u komentaru, informacije se čuvaju u listi nazvanoj `mentions`. Svaki objekat u toj listi
has the following structure.

[inline-code-attrs-start title = 'Objekat spominjanja u komentaru'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike, biće prefiksiran vašim tenant ID-jem. **/
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

Kada se koriste hashtagovi i uspješno se parsiraju, informacije se čuvaju u listi nazvanoj `hashTags`. Svaki objekat u toj listi
has the following structure. Hashtags can also be manually added to the comment `hashTags` array for querying, if `retain` is set.

[inline-code-attrs-start title = 'Objekat hashtag-a komentara'; type = 'typescript'; inline-code-attrs-end]
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

---