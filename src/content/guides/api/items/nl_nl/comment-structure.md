Een `Comment` object vertegenwoordigt een opmerking die door een gebruiker is achtergelaten.

De relatie tussen ouder- en kindreacties wordt gedefinieerd via `parentId`.

De structuur voor het Comment object is als volgt:

[inline-code-attrs-start title = 'Structuur van het Comment-object'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Instellen op true als de spam-engine heeft bepaald dat de opmerking spam is. **/
    aiDeterminedSpam?: boolean
    /** Of de opmerking is goedgekeurd om te tonen. Instellen op true bij het opslaan van de opmerking, anders wordt deze verborgen. **/
    approved?: boolean
    /** De avatar van de gebruiker. **/
    avatarSrc?: string
    /** Child comments. Niet in alle scenario's gevuld. Gebruikt wanneer asTree via de API op true is gezet. **/
    children: Comment[]
    /** De onbewerkte tekst van de opmerking. **/
    comment: string
    /** READONLY: De opmerking van de gebruiker geparseerd naar HTML. **/
    commentHTML?: string
    /** Het e-mailadres van de schrijver. Vereist als anoniem reageren uitgeschakeld is. **/
    commenterEmail?: string
    /** De link van de schrijver (bijv. hun blog). **/
    commenterLink?: string
    /** De naam van de schrijver. Altijd vereist. Als dit niet beschikbaar is, stel het in op iets als "Anoniem". **/
    commenterName: string
    /** De datum waarop de opmerking is geplaatst, in UTC epoch. **/
    date: number
    /** Het "weergavelabel" voor de opmerking - bijvoorbeeld "Admin", "Moderator", of iets als "VIP User". **/
    displayLabel?: string
    /** Het domein waarop de opmerking is geplaatst. **/
    domain?: string
    /** READONLY: Het aantal keer dat de opmerking is gemarkeerd. **/
    flagCount?: number
    /** De #hashtags in de opmerking die succesvol zijn geparseerd. Je kunt ook handmatig hashtags toevoegen voor querydoeleinden, maar deze worden niet automatisch in de tekst van de opmerking weergegeven. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Bevat de opmerking afbeeldingen? **/
    hasImages?: boolean
    /** READONLY: Bevat de opmerking links? **/
    hasLinks?: boolean
    /** READONLY: De unieke id van de opmerking. **/
    id: string
    /** Alleen bij aanmaak! Dit wordt gehasht voor opslag. **/
    ip?: string
    /** READONLY: Heeft de huidige gebruiker de auteur van deze opmerking geblokkeerd? **/
    isBlocked?: boolean
    /** READONLY: Is de opmerking door een admin geplaatst? Automatisch ingesteld op basis van userId. **/
    isByAdmin?: boolean
    /** READONLY: Is de opmerking door een moderator geplaatst? Automatisch ingesteld op basis van userId. **/
    isByModerator?: boolean
    /** Instellen op true als de opmerking soft verwijderd is (er moest een placeholder worden achtergelaten vanwege een andere configuratie). **/
    isDeleted?: boolean
    /** Instellen op true als het account van de gebruiker is verwijderd en de opmerking behouden moest blijven. **/
    isDeletedUser?: boolean
    /** READONLY: Is de opmerking door de momenteel ingelogde gebruiker (contextUserId) gemarkeerd? **/
    isFlagged?: boolean
    /** Is de opmerking vastgezet? **/
    isPinned?: boolean
    /** Is de opmerking vergrendeld? Wanneer true, kan niemand (inclusief moderators) erop reageren, deze bewerken of verwijderen totdat deze ontgrendeld is. **/
    isLocked?: boolean
    /** Is de opmerking spam? **/
    isSpam?: boolean
    /** READONLY: Is de opmerking naar beneden gestemd voor de huidige gebruiker (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: Is de opmerking omhoog gestemd voor de huidige gebruiker (contextUserId)? **/
    isVotedUp?: boolean
    /** De locale van de opmerking. Als deze niet wordt opgegeven, wordt deze afgeleid van de Accept-Language HTTP-header. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: De @mentions in de opmerking die succesvol zijn geparseerd. **/
    mentions?: CommentUserMention[]
    /** Optionele metadata gekoppeld aan de opmerking. **/
    meta?: Record<string, string | number | boolean>
    /** De optionele lijst met moderation group ids die aan deze opmerking zijn gekoppeld. **/
    moderationGroupIds?: string[]|null
    /** READONLY: De id van het stem-object dat overeenkomt met de stem van de huidige gebruiker (contextUserId) op deze opmerking. **/
    myVoteId?: string
    /** Of er notificaties zijn verzonden voor deze opmerking aan commentatoren. Om te voorkomen dat notificaties bij imports worden verzonden, zet dit op true. **/
    notificationSentForParent?: boolean
    /** Of er notificaties zijn verzonden voor deze opmerking aan tenant-gebruikers. Om te voorkomen dat notificaties bij imports worden verzonden, zet dit op true. **/
    notificationSentForParentTenant?: boolean
    /** De titel van de pagina waarop deze opmerking stond. **/
    pageTitle?: string
    /** Als we op een opmerking reageren, is dit de ID waarop we reageren. **/
    parentId?: string|null
    /** Of de opmerking als beoordeeld is gemarkeerd. **/
    reviewed: boolean
    /** De tenant id waar deze opmerking bij hoort. **/
    tenantId: string
    /** De gebruiker die de opmerking schreef. Wordt automatisch aangemaakt bij het opslaan van een opmerking met een naam/e-mail. **/
    userId?: string|null
    /** De URL naar de locatie waar deze opmerking zichtbaar is, zoals een blogpost. **/
    url: string
    /** Een "opgeschoonde" versie van de urlId die je aan ons doorgaf. Bij het opslaan geef je dit veld op, maar bij het ophalen van de opmerking wordt dit "opgeschoond" en wordt je originele waarde verplaatst naar "urlIdRaw". **/
    urlId: string
    /** READONLY: De originele urlId die je aan ons doorgaf. **/
    urlIdRaw?: string
    /** Is de gebruiker en deze opmerking geverifieerd? **/
    verified: boolean
    /** Aantal stemmen omhoog. **/
    votesUp?: number
    /** Aantal stemmen omlaag. **/
    votesDown?: number
    /** De "karma" van de opmerking (= stemmen omhoog - stemmen omlaag). **/
    votes?: number
}
[inline-code-end]

Sommige van deze velden zijn gemarkeerd als `READONLY` - deze worden door de API teruggegeven maar kunnen niet worden ingesteld.

### Comment Text Structure

Reacties worden geschreven in een FastComments-variant van markdown, wat gewoon markdown is plus traditionele `bbcode`-stijl tags voor afbeeldingen, zoals `[img]path[/img]`.

Tekst wordt in twee velden opgeslagen. De tekst die de gebruiker invoerde, wordt ongewijzigd opgeslagen in het veld `comment`. Deze wordt gerenderd en opgeslagen in het veld `commentHTML`.

De toegestane HTML-tags zijn `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Het wordt aanbevolen om de HTML te renderen; omdat het een heel kleine subset van HTML is, is het vrij eenvoudig om een renderer te bouwen. Er zijn bijvoorbeeld meerdere bibliotheken voor React Native en Flutter die hierbij kunnen helpen

Je kunt ervoor kiezen om de niet-genormaliseerde waarde van het veld `comment` te renderen. [Een voorbeeld-parser vind je hier.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

De voorbeeld-parser kan ook worden aangepast om met HTML te werken en de HTML-tags om te zetten naar de verwachte elementen voor jouw platform. 

### Tagging

Wanneer gebruikers in een opmerking worden getagd, wordt die informatie opgeslagen in een lijst genaamd `mentions`. Elk object in die lijst
heeft de volgende structuur.

[inline-code-attrs-start title = 'Het Comment Mentions-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** De user-id. Voor SSO-gebruikers heeft dit je tenant-id als prefix. **/
    id: string
    /** De uiteindelijke @mention-tagtekst, inclusief het @-symbool. **/
    tag: string
    /** De oorspronkelijke @mention-tagtekst, inclusief het @-symbool. **/
    rawTag: string
    /** Welk type gebruiker is getagd. user = FastComments.com-account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Als de gebruiker zich afmeldt voor notificaties, blijft dit toch op true staan. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Wanneer hashtags worden gebruikt en succesvol geparseerd, wordt die informatie opgeslagen in een lijst genaamd `hashTags`. Elk object in die lijst
heeft de volgende structuur. Hashtags kunnen ook handmatig aan de `hashTags`-array van de opmerking worden toegevoegd voor querydoeleinden, als `retain` is ingesteld.

[inline-code-attrs-start title = 'Het Comment HashTag-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** De hashtag-id. **/
    id: string
    /** De uiteindelijke #hashtag-tagtekst, inclusief het #-symbool. **/
    tag: string
    /** Als de hashtag is gekoppeld aan een aangepaste URL, wordt deze opgegeven. **/
    url?: string
    /** Of we de hashtag moeten behouden, zelfs als deze niet in de opmerkingstekst voorkomt wanneer de opmerking wordt bijgewerkt. Handig om opmerkingen te taggen zonder de tekst te wijzigen. **/
    retain?: boolean
}
[inline-code-end]

---