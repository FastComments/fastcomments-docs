Et `Comment` objekt repræsenterer en kommentar efterladt af en bruger.

Forholdet mellem forældre- og underkommentarer defineres via `parentId`.

Strukturen for Comment-objektet er som følger:

[inline-code-attrs-start title = 'Kommentarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** KUN LÆS: Sættes til true hvis spam-motoren vurderede kommentaren som spam. **/
    aiDeterminedSpam?: boolean
    /** Om kommentaren er godkendt til visning. Sættes til true ved lagring af kommentaren, ellers skjules den. **/
    approved?: boolean
    /** Brugerens avatar. **/
    avatarSrc?: string
    /** Underkommentarer. Ikke udfyldt i alle scenarier. Bruges når asTree er sat til true via API'et. **/
    children: Comment[]
    /** Kommentatorens rå kommentar. **/
    comment: string
    /** KUN LÆS: Kommentatorens kommentar parseret til HTML. **/
    commentHTML?: string
    /** Kommentatorens e-mail. Kræves hvis anonym kommentering er slået fra. **/
    commenterEmail?: string
    /** Kommentatorens link (fx deres blog). **/
    commenterLink?: string
    /** Kommentatorens navn. Altid påkrævet. Hvis ikke tilgængeligt, sæt til noget som "Anonymous". **/
    commenterName: string
    /** Datoen kommentaren blev efterladt, i UTC epoch. **/
    date: number
    /** "Display label" for kommentaren - for eksempel "Admin", "Moderator", eller noget som "VIP User". **/
    displayLabel?: string
    /** Domænet kommentaren blev postet på. **/
    domain?: string
    /** KUN LÆS: Antallet af gange kommentaren er blevet rapporteret. **/
    flagCount?: number
    /** De #hashtags skrevet i kommentaren, som blev succesfuldt parset. Du kan også manuelt tilføje hashtags til forespørgsler, men de vil ikke automatisk vises i kommentarens tekst. **/
    hashTags?: CommentHashTag[]
    /** KUN LÆS: Indeholder kommentaren billeder? **/
    hasImages?: boolean
    /** KUN LÆS: Indeholder kommentaren links? **/
    hasLinks?: boolean
    /** KUN LÆS: Den unikke kommentar id. **/
    id: string
    /** Kun ved oprettelse! Dette hashes til lagring. **/
    ip?: string
    /** KUN LÆS: Har den aktuelle bruger blokeret brugeren der skrev denne kommentar? **/
    isBlocked?: boolean
    /** KUN LÆS: Er kommentaren af en admin? Sættes automatisk baseret på userId. **/
    isByAdmin?: boolean
    /** KUN LÆS: Er kommentaren af en moderator? Sættes automatisk baseret på userId. **/
    isByModerator?: boolean
    /** Sættes til true hvis kommentaren blev blødt slettet (en pladsholder måtte bevares pga. anden konfiguration). **/
    isDeleted?: boolean
    /** Sættes til true hvis brugerens konto blev slettet og kommentaren måtte bevares. **/
    isDeletedUser?: boolean
    /** KUN LÆS: Er kommentaren markeret af den nuværende loggede bruger (contextUserId)? **/
    isFlagged?: boolean
    /** Er kommentaren fastgjort? **/
    isPinned?: boolean
    /** Er kommentaren låst? Når true, kan ingen (inkl. moderatorer) svare, redigere eller slette den, indtil den låses op. **/
    isLocked?: boolean
    /** Er kommentaren spam? **/
    isSpam?: boolean
    /** KUN LÆS: Er kommentaren nedstemt af den aktuelle bruger (contextUserId)? **/
    isVotedDown?: boolean
    /** KUN LÆS: Er kommentaren opstemmet af den aktuelle bruger (contextUserId)? **/
    isVotedUp?: boolean
    /** Den locale kommentaren er i. Hvis ikke angivet, vil den blive udledt fra 'Accept-Language' HTTP-headeren. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** KUN LÆS: De @mentions skrevet i kommentaren, som blev succesfuldt parset. **/
    mentions?: CommentUserMention[]
    /** Valgfri metadata associeret med kommentaren. **/
    meta?: Record<string, string | number | boolean>
    /** Den valgfrie liste af moderationsgruppe-id'er associeret med denne kommentar. **/
    moderationGroupIds?: string[]|null
    /** KUN LÆS: Id'et på vote-objektet som svarer til stemmen fra den aktuelle bruger (contextUserId) på denne kommentar. **/
    myVoteId?: string
    /** Om notifikationer blev sendt for denne kommentar til kommentatorer. For at forhindre notifikationer ved import, sæt denne til true. **/
    notificationSentForParent?: boolean
    /** Om notifikationer blev sendt for denne kommentar til tenant-brugere. For at forhindre notifikationer ved import, sæt denne til true. **/
    notificationSentForParentTenant?: boolean
    /** Titlen på siden kommentaren var på. **/
    pageTitle?: string
    /** Hvis vi svarer på en kommentar, er dette ID'et vi svarer til. **/
    parentId?: string|null
    /** Om kommentaren er markeret som gennemset (reviewed). **/
    reviewed: boolean
    /** Tenant-id'et som kommentaren hører til. **/
    tenantId: string
    /** Brugeren der skrev kommentaren. Oprettes automatisk ved lagring af en kommentar med navn/e-mail. **/
    userId?: string|null
    /** URL'en til stedet hvor denne kommentar er synlig, fx et blogindlæg. **/
    url: string
    /** En "renset" version af den urlId du sendte os. Ved lagring specificerer du dette felt, men når du henter kommentaren tilbage, bliver dette "renset" og din oprindelige værdi flyttet til "urlIdRaw". **/
    urlId: string
    /** KUN LÆS: Den oprindelige urlId du sendte os. **/
    urlIdRaw?: string
    /** Er brugeren og denne kommentar verificeret? **/
    verified: boolean
    /** Antal up-stemmer. **/
    votesUp?: number
    /** Antal ned-stemmer. **/
    votesDown?: number
    /** Kommentarens "karma" (= votes up - votes down). **/
    votes?: number
}
[inline-code-end]

Nogle af disse felter er markeret `READONLY` - disse returneres af API'et, men kan ikke sættes.

### Kommentarens tekststruktur

Kommentarer skrives i en FastComments-variant af markdown, som er markdown plus traditionelle `bbcode`-lignende tags til billeder, fx `[img]path[/img]`.

Tekst gemmes i to felter. Teksten brugeren skrev gemmes uændret i feltet `comment`. Denne renderes og gemmes i feltet `commentHTML`.

De tilladte HTML-tags er `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Det anbefales at rendre HTML'en, da det er et meget lille udsnit af HTML, og det er forholdsvis ligetil at bygge en renderer. Der findes flere biblioteker til blandt andet React Native og Flutter, som kan hjælpe med dette.

Du kan vælge at rendre den ikke-normaliserede værdi af feltet `comment`. [Et eksempel på en parser findes her.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Eksempelparseren kan også justeres til at arbejde med HTML og transformere HTML-tags til de forventede elementer der skal rendres på din platform. 

### Mærkning

Når brugere bliver tagget i en kommentar, gemmes informationen i en liste kaldet `mentions`. Hvert objekt i den liste har følgende struktur.

[inline-code-attrs-start title = 'Objektet for kommentar-omtaler'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Bruger-id. For SSO-brugere vil dette have dit tenant-id som præfiks. **/
    id: string
    /** Den endelige @mention-tag tekst, inkl. @-symbolet. **/
    tag: string
    /** Den oprindelige @mention-tag tekst, inkl. @-symbolet. **/
    rawTag: string
    /** Hvilken type bruger blev tagget. user = FastComments.com-konto. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Hvis brugeren fravælger notifikationer, vil dette stadig blive sat til true. **/
    sent: boolean
}
[inline-code-end]

### Hashtags

Når hashtags bruges og succesfuldt parses, gemmes informationen i en liste kaldet `hashTags`. Hvert objekt i den liste har følgende struktur. Hashtags kan også manuelt tilføjes til kommentarens `hashTags`-array for forespørgsler, hvis `retain` er sat.

[inline-code-attrs-start title = 'Objektet for kommentar-hashtags'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** Hashtag-id. **/
    id: string
    /** Den endelige #hashtag-tag tekst, inkl. #-symbolet. **/
    tag: string
    /** Hvis hashtaggen er associeret med en brugerdefineret URL, vil dette være defineret. **/
    url?: string
    /** Hvis vi skal bevare hashtaggen, selvom den ikke findes i kommentarens tekst, når kommentaren opdateres. Nyttigt for at tagge kommentarer uden at ændre kommentarens tekst. **/
    retain?: boolean
}
[inline-code-end]

---