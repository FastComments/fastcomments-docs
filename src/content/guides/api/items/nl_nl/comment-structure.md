Een `Comment` object stelt een opmerking voor die door een gebruiker is achtergelaten.

De relatie tussen ouder- en kindreacties wordt gedefinieerd via `parentId`.

De structuur van het Comment-object is als volgt:

[inline-code-attrs-start title = 'Structuur van het Comment-object'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Zet op true als de spam-engine heeft vastgesteld dat het commentaar spam is. **/
    aiDeterminedSpam?: boolean
    /** Of het commentaar is goedgekeurd om te tonen. Zet op true bij het opslaan van het commentaar, anders wordt het verborgen. **/
    approved?: boolean
    /** De avatar van de gebruiker. **/
    avatarSrc?: string
    /** Kindreacties. Niet in alle scenario's gevuld. Gebruikt wanneer asTree via de API op true is gezet. **/
    children: Comment[]
    /** De onbewerkte opmerking van de schrijver. **/
    comment: string
    /** READONLY: De opmerking van de schrijver geparseerd naar HTML. **/
    commentHTML?: string
    /** Het e-mailadres van de schrijver. Vereist als anoniem reageren is uitgeschakeld. **/
    commenterEmail?: string
    /** De link van de schrijver (bijv. hun blog). **/
    commenterLink?: string
    /** De naam van de schrijver. Altijd vereist. Als niet beschikbaar, zet iets als "Anoniem". **/
    commenterName: string
    /** De datum waarop de opmerking is geplaatst, in UTC epoch. **/
    date: number
    /** Het "weergavelabel" voor het commentaar - bijvoorbeeld "Admin", "Moderator", of iets als "VIP User". **/
    displayLabel?: string
    /** Het domein waarop de opmerking is geplaatst. **/
    domain?: string
    /** READONLY: Het aantal keer dat het commentaar is gemarkeerd. **/
    flagCount?: number
    /** De #hashtags die in de opmerking zijn geschreven en succesvol zijn geparseerd. Je kunt ook hashtags handmatig toevoegen voor query's, maar deze worden niet automatisch in de opmerkingstekst weergegeven. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Bevat de opmerking afbeeldingen? **/
    hasImages?: boolean
    /** READONLY: Bevat de opmerking links? **/
    hasLinks?: boolean
    /** READONLY: De unieke id van het commentaar. **/
    id: string
    /** Alleen bij aanmaak! Dit wordt gehasht voor opslag. **/
    ip?: string
    /** READONLY: Heeft de huidige gebruiker de auteur van dit commentaar geblokkeerd? **/
    isBlocked?: boolean
    /** READONLY: Is het commentaar door een admin geschreven? Automatisch ingesteld op basis van userId. **/
    isByAdmin?: boolean
    /** READONLY: Is het commentaar door een moderator geschreven? Automatisch ingesteld op basis van userId. **/
    isByModerator?: boolean
    /** Zet op true als het commentaar zacht verwijderd is (er moest een placeholder blijven vanwege een andere configuratie). **/
    isDeleted?: boolean
    /** Zet op true als het account van de gebruiker is verwijderd en het commentaar moest worden behouden. **/
    isDeletedUser?: boolean
    /** READONLY: Is het gemarkeerd door de momenteel ingelogde gebruiker (contextUserId)? **/
    isFlagged?: boolean
    /** Is het commentaar vastgezet? **/
    isPinned?: boolean
    /** Is het commentaar vergrendeld voor nieuwe reacties (moderators kunnen nog steeds reageren)? **/
    isLocked?: boolean
    /** Is het commentaar spam? **/
    isSpam?: boolean
    /** READONLY: Is het commentaar naar beneden gestemd voor de huidige gebruiker (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: Is het commentaar omhoog gestemd voor de huidige gebruiker (contextUserId)? **/
    isVotedUp?: boolean
    /** De locale van het commentaar. Indien niet opgegeven, wordt deze afgeleid van de Accept-Language HTTP-header. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: De @mentions in het commentaar die succesvol zijn geparseerd. **/
    mentions?: CommentUserMention[]
    /** Optionele metadata geassocieerd met het commentaar. **/
    meta?: Record<string, string | number | boolean>
    /** De optionele lijst met moderation group ids die aan dit commentaar zijn gekoppeld. **/
    moderationGroupIds?: string[]|null
    /** READONLY: De id van het stemobject dat overeenkomt met de stem van de huidige gebruiker (contextUserId) op dit commentaar. **/
    myVoteId?: string
    /** Of er meldingen zijn verzonden voor dit commentaar naar reageerders. Om te voorkomen dat meldingen worden verzonden bij imports, zet dit op true. **/
    notificationSentForParent?: boolean
    /** Of er meldingen zijn verzonden voor dit commentaar naar tenant-gebruikers. Om te voorkomen dat meldingen worden verzonden bij imports, zet dit op true. **/
    notificationSentForParentTenant?: boolean
    /** De titel van de pagina waarop dit commentaar stond. **/
    pageTitle?: string
    /** Als we op een opmerking reageren, is dit de ID waarop we reageren. **/
    parentId?: string|null
    /** Of het commentaar als beoordeeld is gemarkeerd. **/
    reviewed: boolean
    /** De tenant-id waartoe het commentaar behoort. **/
    tenantId: string
    /** De gebruiker die het commentaar heeft geschreven. Automatisch aangemaakt bij het opslaan van een commentaar met een naam/e-mail. **/
    userId?: string|null
    /** De URL naar de locatie waarop dit commentaar zichtbaar is, zoals een blogpost. **/
    url: string
    /** Een "opgeschoonde" versie van de urlId die je aan ons doorgeeft. Bij opslaan geef je dit veld op, maar bij ophalen wordt dit "opgeschoond" en wordt je oorspronkelijke waarde verplaatst naar "urlIdRaw". **/
    urlId: string
    /** READONLY: De oorspronkelijke urlId die je ons hebt gegeven. **/
    urlIdRaw?: string
    /** Is de gebruiker en dit commentaar geverifieerd? **/
    verified: boolean
    /** Aantal stemmen omhoog. **/
    votesUp?: number
    /** Aantal stemmen omlaag. **/
    votesDown?: number
    /** De "karma" van het commentaar (= stemmen omhoog - stemmen omlaag). **/
    votes?: number
}
[inline-code-end]

Sommige van deze velden zijn gemarkeerd als `READONLY` - deze worden door de API geretourneerd maar kunnen niet worden ingesteld.

### Structuur van de opmerkingstekst

Opmerkingen worden geschreven in een FastComments-variant van markdown, wat gewoon markdown is plus traditionele `bbcode`-stijl tags voor afbeeldingen, zoals `[img]path[/img]`.

Tekst wordt in twee velden opgeslagen. De tekst die de gebruiker invoerde wordt ongewijzigd opgeslagen in het veld `comment`. Dit wordt gerenderd en opgeslagen in het veld `commentHTML`.

De toegestane HTML-tags zijn `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Het wordt aangeraden de HTML te renderen, aangezien het een zeer kleine subset van HTML is, het bouwen van een renderer is vrij eenvoudig. Er zijn bijvoorbeeld meerdere libraries voor React Native en Flutter die hierbij kunnen helpen.

Je kunt ervoor kiezen de niet-genormaliseerde waarde van het veld `comment` te renderen. [Een voorbeeldparser vind je hier.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

De voorbeeldparser kan ook aangepast worden om met HTML te werken en de HTML-tags om te zetten naar de verwachte elementen voor rendering op jouw platform.

### Tagging

Wanneer gebruikers in een opmerking worden getagd, wordt de informatie opgeslagen in een lijst genaamd `mentions`. Elk object in die lijst heeft de volgende structuur.

[inline-code-attrs-start title = 'Het Comment Vermeldingen-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** De gebruikers-id. Voor SSO-gebruikers wordt hier je tenant-id als prefix toegevoegd. **/
    id: string
    /** De uiteindelijke @mention tekst, inclusief het @-symbool. **/
    tag: string
    /** De oorspronkelijke @mention tekst, inclusief het @-symbool. **/
    rawTag: string
    /** Welk type gebruiker is getagd. user = FastComments.com-account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Als de gebruiker zich afmeldt voor meldingen, wordt dit nog steeds op true gezet. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Wanneer hashtags worden gebruikt en succesvol geparseerd, wordt de informatie opgeslagen in een lijst genaamd `hashTags`. Elk object in die lijst heeft de volgende structuur. Hashtags kunnen ook handmatig aan de `hashTags`-array van een opmerking worden toegevoegd voor query-doeleinden, als `retain` is ingesteld.

[inline-code-attrs-start title = 'Het Comment HashTag-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** De hashtag-id. **/
    id: string
    /** De uiteindelijke #hashtag tekst, inclusief het #-symbool. **/
    tag: string
    /** Als de hashtag is gekoppeld aan een aangepaste URL, wordt deze hier gedefinieerd. **/
    url?: string
    /** Of we de hashtag moeten behouden, ook als deze niet in de opmerkingstekst voorkomt, wanneer de opmerking wordt bijgewerkt. Handig om opmerkingen te taggen zonder de tekst te wijzigen. **/
    retain?: boolean
}
[inline-code-end]

---