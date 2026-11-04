Een `Comment` object vertegenwoordigt een reactie die door een gebruiker is achtergelaten.

De relatie tussen bovenliggende en onderliggende reacties wordt gedefinieerd via `parentId`.

De structuur van het Comment object is als volgt:

[inline-code-attrs-start title = 'Comment Structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Ingesteld op true als de spam-engine heeft bepaald dat de reactie spam was. **/
    aiDeterminedSpam?: boolean
    /** Of de reactie is goedgekeurd om te tonen. Ingesteld op true bij het opslaan van de reactie, anders wordt deze verborgen. **/
    approved?: boolean
    /** De avatar van de gebruiker. **/
    avatarSrc?: string
    /** Onderliggende reacties. Niet in alle scenario's gevuld. Gebruikt wanneer asTree via de API op true is ingesteld. **/
    children: Comment[]
    /** De ruwe reactie van de reageerder. **/
    comment: string
    /** READONLY: De reactie van de reageerder, geparseerd naar HTML. **/
    commentHTML?: string
    /** Het e-mailadres van de reageerder. Verplicht als anoniem reageren is uitgeschakeld. **/
    commenterEmail?: string
    /** De link van de reageerder (bijvoorbeeld zijn of haar blog). **/
    commenterLink?: string
    /** De naam van de reageerder. Altijd verplicht. Indien niet beschikbaar, stel dan iets in als "Anoniem". **/
    commenterName: string
    /** De datum waarop de reactie is achtergelaten, in UTC epoch. **/
    date: number
    /** Het "weergavelabel" voor de reactie - bijvoorbeeld "Admin", "Moderator", of iets als "VIP-gebruiker". **/
    displayLabel?: string
    /** Het domein waarop de reactie is geplaatst. **/
    domain?: string
    /** READONLY: Het aantal keren dat de reactie is gemarkeerd (gevlagd). **/
    flagCount?: number
    /** De #hashtags die in de reactie zijn geschreven en succesvol zijn geparseerd. Je kunt ook handmatig hashtags toevoegen, bijvoorbeeld voor het opvragen van reacties, maar deze worden niet automatisch in de reactietekst weergegeven. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Bevat de reactie afbeeldingen? **/
    hasImages?: boolean
    /** READONLY: Bevat de reactie links? **/
    hasLinks?: boolean
    /** READONLY: De unieke reactie-id. **/
    id: string
    /** Alleen bij aanmaken! Deze wordt gehasht voor opslag. **/
    ip?: string
    /** READONLY: Heeft de huidige gebruiker de gebruiker die deze reactie heeft geschreven geblokkeerd? **/
    isBlocked?: boolean
    /** READONLY: Is de reactie van een admin? Automatisch ingesteld op basis van userId. **/
    isByAdmin?: boolean
    /** READONLY: Is de reactie van een moderator? Automatisch ingesteld op basis van userId. **/
    isByModerator?: boolean
    /** Ingesteld op true als de reactie soft is verwijderd (er moest een placeholder achterblijven vanwege een andere configuratie). **/
    isDeleted?: boolean
    /** Ingesteld op true als het account van de gebruiker is verwijderd en de reactie behouden moest blijven. **/
    isDeletedUser?: boolean
    /** READONLY: Is de reactie gemarkeerd door de momenteel ingelogde gebruiker (contextUserId)? **/
    isFlagged?: boolean
    /** Is de reactie vastgepind? **/
    isPinned?: boolean
    /** Is de reactie vergrendeld? Indien true, kan niemand (inclusief moderators) erop reageren, deze bewerken of verwijderen totdat deze is ontgrendeld. **/
    isLocked?: boolean
    /** Is de reactie spam? **/
    isSpam?: boolean
    /** READONLY: Is de reactie negatief gestemd door de huidige gebruiker (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: Is de reactie positief gestemd door de huidige gebruiker (contextUserId)? **/
    isVotedUp?: boolean
    /** De taalinstelling (locale) van de reactie. Indien niet opgegeven, wordt deze afgeleid uit de HTTP Accept-Language header. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: De @mentions die in de reactie zijn geschreven en succesvol zijn geparseerd. **/
    mentions?: CommentUserMention[]
    /** Optionele metadata gekoppeld aan de reactie. **/
    meta?: Record<string, string | number | boolean>
    /** De optionele lijst met moderatiegroep-id's die aan deze reactie zijn gekoppeld. **/
    moderationGroupIds?: string[]|null
    /** READONLY: De id van het stemobject dat hoort bij de stem van de huidige gebruiker (contextUserId) op deze reactie. **/
    myVoteId?: string
    /** Of er meldingen zijn verstuurd voor deze reactie naar reageerders. Om te voorkomen dat er meldingen worden verstuurd bij imports, stel dit in op true. **/
    notificationSentForParent?: boolean
    /** Of er meldingen zijn verstuurd voor deze reactie naar tenant-gebruikers. Om te voorkomen dat er meldingen worden verstuurd bij imports, stel dit in op true. **/
    notificationSentForParentTenant?: boolean
    /** De titel van de pagina waarop deze reactie stond. **/
    pageTitle?: string
    /** Als we op een reactie antwoorden, is dit de ID waarop we antwoorden. **/
    parentId?: string|null
    /** Of de reactie als beoordeeld is gemarkeerd. **/
    reviewed: boolean
    /** De tenant-id waartoe de reactie behoort. **/
    tenantId: string
    /** De gebruiker die de reactie heeft geschreven. Wordt automatisch aangemaakt bij het opslaan van een reactie met een naam/e-mailadres. **/
    userId?: string|null
    /** De URL naar de locatie waar deze reactie zichtbaar is, zoals een blogpost. **/
    url: string
    /** Een "opgeschoonde" versie van de urlId die je ons hebt doorgegeven. Bij het opslaan geef je dit veld op, maar wanneer je de reactie weer ophaalt is deze "opgeschoond" en is je oorspronkelijke waarde verplaatst naar "urlIdRaw". **/
    urlId: string
    /** READONLY: De oorspronkelijke urlId die je ons hebt doorgegeven. **/
    urlIdRaw?: string
    /** Is de gebruiker en deze reactie geverifieerd? **/
    verified: boolean
    /** Aantal positieve stemmen. **/
    votesUp?: number
    /** Aantal negatieve stemmen. **/
    votesDown?: number
    /** De "karma" van de reactie (= positieve stemmen - negatieve stemmen). **/
    votes?: number
}
[inline-code-end]

Sommige van deze velden zijn gemarkeerd als `READONLY` - deze worden door de API geretourneerd maar kunnen niet worden ingesteld.

### Structuur van de reactietekst

Reacties worden geschreven in een FastComments-variant van markdown, wat gewoon markdown is plus traditionele `bbcode`-stijl tags voor afbeeldingen, zoals `[img]path[/img]`.

De tekst wordt in twee velden opgeslagen. De tekst die de gebruiker heeft ingevoerd wordt ongewijzigd opgeslagen in het `comment`-veld. Deze wordt gerenderd en opgeslagen in het `commentHTML`-veld.

De toegestane HTML-tags zijn `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Het wordt aanbevolen om de HTML te renderen, aangezien het een zeer kleine subset van HTML is; het bouwen van een renderer is vrij eenvoudig. Er zijn bijvoorbeeld meerdere bibliotheken voor React Native en Flutter die hierbij kunnen helpen.

Je kunt er ook voor kiezen om de niet-genormaliseerde waarde van het `comment`-veld te renderen. [Een voorbeeld-parser is hier te vinden.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

De voorbeeld-parser kan ook worden aangepast om met HTML te werken, en de HTML-tags om te zetten naar de verwachte elementen om te renderen op jouw platform.

### Taggen

Wanneer gebruikers in een reactie worden getagd, wordt de informatie opgeslagen in een lijst genaamd `mentions`. Elk object in die lijst
heeft de volgende structuur.

[inline-code-attrs-start title = 'Het Comment Mentions Object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** De gebruikers-id. Voor SSO-gebruikers wordt hier je tenant-id aan voorafgegaan. **/
    id: string
    /** De uiteindelijke @mention-tagtekst, inclusief het @-symbool. **/
    tag: string
    /** De oorspronkelijke @mention-tagtekst, inclusief het @-symbool. **/
    rawTag: string
    /** Welk type gebruiker is getagd. user = FastComments.com-account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Als de gebruiker zich afmeldt voor meldingen, wordt dit nog steeds op true ingesteld. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Wanneer hashtags worden gebruikt en succesvol zijn geparseerd, wordt de informatie opgeslagen in een lijst genaamd `hashTags`. Elk object in die lijst
heeft de volgende structuur. Hashtags kunnen ook handmatig worden toegevoegd aan de `hashTags`-array van de reactie, bijvoorbeeld voor het opvragen van reacties, als `retain` is ingesteld.

[inline-code-attrs-start title = 'Het Comment HashTag Object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** De hashtag-id. **/
    id: string
    /** De uiteindelijke #hashtag-tagtekst, inclusief het #-symbool. **/
    tag: string
    /** Als de hashtag is gekoppeld aan een aangepaste URL, wordt deze gedefinieerd. **/
    url?: string
    /** Als we de hashtag moeten behouden, zelfs als deze niet in de reactietekst voorkomt, wanneer de reactie wordt bijgewerkt. Handig voor het taggen van reacties zonder de reactietekst te wijzigen. **/
    retain?: boolean
}
[inline-code-end]