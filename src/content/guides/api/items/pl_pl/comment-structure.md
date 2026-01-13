A `Comment` object represents a comment left by a user.

The relationship between parent and child comments is defined via `parentId`.

The structure for the Comment object is as follows:

[inline-code-attrs-start title = 'Struktura komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** TYLKO DO ODCZYTU: Ustawione na true, jeśli silnik antyspamowy uznał komentarz za spam. **/
    aiDeterminedSpam?: boolean
    /** Czy komentarz jest zatwierdzony do wyświetlania. Ustawione na true podczas zapisywania komentarza, w przeciwnym razie będzie ukryty. **/
    approved?: boolean
    /** Avatar użytkownika. **/
    avatarSrc?: string
    /** Komentarze podrzędne. Nie są wypełniane we wszystkich scenariuszach. Używane, gdy asTree jest ustawione na true przez API. **/
    children: Comment[]
    /** Surowy komentarz autora. **/
    comment: string
    /** TYLKO DO ODCZYTU: Komentarz autora sparsowany do HTML. **/
    commentHTML?: string
    /** Email autora komentarza. Wymagany, jeśli komentowanie anonimowe jest wyłączone. **/
    commenterEmail?: string
    /** Link autora komentarza (np. ich blog). **/
    commenterLink?: string
    /** Imię autora komentarza. Zawsze wymagane. Jeśli niedostępne, ustaw na coś w rodzaju "Anonymous". **/
    commenterName: string
    /** Data pozostawienia komentarza, w formacie epoki UTC. **/
    date: number
    /** "Etykieta wyświetlana" komentarza - na przykład "Admin", "Moderator", lub coś w stylu "VIP User". **/
    displayLabel?: string
    /** Domenę, na której opublikowano komentarz. **/
    domain?: string
    /** TYLKO DO ODCZYTU: Liczba razy, kiedy komentarz został oznaczony (zgłoszony). **/
    flagCount?: number
    /** #hashtagi napisane w komentarzu, które zostały pomyślnie sparsowane. Można też ręcznie dodać hashtagi do zapytania, ale nie będą one automatycznie wyświetlane w treści komentarza. **/
    hashTags?: CommentHashTag[]
    /** TYLKO DO ODCZYTU: Czy komentarz zawiera obrazy? **/
    hasImages?: boolean
    /** TYLKO DO ODCZYTU: Czy komentarz zawiera linki? **/
    hasLinks?: boolean
    /** TYLKO DO ODCZYTU: Unikalne id komentarza. **/
    id: string
    /** Tylko podczas tworzenia! To jest haszowane do przechowywania. **/
    ip?: string
    /** TYLKO DO ODCZYTU: Czy bieżący użytkownik zablokował użytkownika, który napisał ten komentarz? **/
    isBlocked?: boolean
    /** TYLKO DO ODCZYTU: Czy komentarz jest napisany przez administratora? Ustawiane automatycznie na podstawie userId. **/
    isByAdmin?: boolean
    /** TYLKO DO ODCZYTU: Czy komentarz jest napisany przez moderatora? Ustawiane automatycznie na podstawie userId. **/
    isByModerator?: boolean
    /** Ustawione na true, jeśli komentarz został miękko usunięty (zostawiono zastępczy wpis z powodu innej konfiguracji). **/
    isDeleted?: boolean
    /** Ustawione na true, jeśli konto użytkownika zostało usunięte, a komentarz musiał zostać zachowany. **/
    isDeletedUser?: boolean
    /** TYLKO DO ODCZYTU: Czy został oznaczony przez aktualnie zalogowanego użytkownika (contextUserId)? **/
    isFlagged?: boolean
    /** Czy komentarz jest przypięty? **/
    isPinned?: boolean
    /** Czy komentarz jest zablokowany dla nowych odpowiedzi (moderatorzy nadal mogą odpowiadać)? **/
    isLocked?: boolean
    /** Czy komentarz jest spamem? **/
    isSpam?: boolean
    /** TYLKO DO ODCZYTU: Czy komentarz został oceniony negatywnie przez bieżącego użytkownika (contextUserId)? **/
    isVotedDown?: boolean
    /** TYLKO DO ODCZYTU: Czy komentarz został oceniony pozytywnie przez bieżącego użytkownika (contextUserId)? **/
    isVotedUp?: boolean
    /** Lokalizacja/język komentarza. Jeśli nie podano, zostanie wywnioskowana z nagłówka HTTP Accept-Language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** TYLKO DO ODCZYTU: Wzmianki @ napisane w komentarzu, które zostały pomyślnie sparsowane. **/
    mentions?: CommentUserMention[]
    /** Opcjonalne metadane powiązane z komentarzem. **/
    meta?: Record<string, string | number | boolean>
    /** Opcjonalna lista id grup moderacyjnych powiązanych z tym komentarzem. **/
    moderationGroupIds?: string[]|null
    /** TYLKO DO ODCZYTU: Id obiektu głosowania odpowiadającego głosowi bieżącego użytkownika (contextUserId) na tym komentarzu. **/
    myVoteId?: string
    /** Czy powiadomienia zostały wysłane dla tego komentarza do komentujących. Aby zapobiec wysyłaniu powiadomień podczas importu, ustaw to na true. **/
    notificationSentForParent?: boolean
    /** Czy powiadomienia zostały wysłane dla tego komentarza do użytkowników tenantów. Aby zapobiec wysyłaniu powiadomień podczas importu, ustaw to na true. **/
    notificationSentForParentTenant?: boolean
    /** Tytuł strony, na której znajdował się ten komentarz. **/
    pageTitle?: string
    /** Jeśli odpowiadamy na komentarz, to jest ID komentarza, na który odpowiadamy. **/
    parentId?: string|null
    /** Czy komentarz jest oznaczony jako przejrzany. **/
    reviewed: boolean
    /** Id tenanta, do którego należy komentarz. **/
    tenantId: string
    /** Użytkownik, który napisał komentarz. Tworzony automatycznie podczas zapisywania komentarza z nazwą/email. **/
    userId?: string|null
    /** URL do miejsca, gdzie komentarz jest widoczny, np. wpis na blogu. **/
    url: string
    /** "Oczyszczona" wersja urlId, które przekazałeś. Podczas zapisu określasz to pole, ale po pobraniu komentarza zostanie ono "oczyszczone", a twoja oryginalna wartość przeniesiona do "urlIdRaw". **/
    urlId: string
    /** TYLKO DO ODCZYTU: Oryginalne urlId, które przekazałeś. **/
    urlIdRaw?: string
    /** Czy użytkownik i ten komentarz są zweryfikowani? **/
    verified: boolean
    /** Liczba głosów za. **/
    votesUp?: number
    /** Liczba głosów przeciw. **/
    votesDown?: number
    /** "Karma" komentarza (= głosy za - głosy przeciw). **/
    votes?: number
}
[inline-code-end]

Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.

### Struktura tekstu komentarza

Comments are written in a FastComments flavor of markdown, which is just markdown plus traditional `bbcode` style tags for images, like `[img]path[/img]`.

Text is stored in two fields. The text the user entered is stored unmodified in the `comment` field. This is rendered and stored in the `commentHTML` field.

The allowed HTML tags are `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

It's recommended to render the HTML, since it is a very small subset of HTML, building a renderer is pretty straightforward. There are multiple libraries for React Native and Flutter, for instance, to help with this

You may choose to render the un-normalized value of the `comment` field. [An example parser is here.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

The example parser could also be adjusted to work with HTML, and transform the HTML tags into expected elements to render for your platform. 

### Oznaczanie

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Obiekt wzmianki komentarza'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id użytkownika. Dla użytkowników SSO będzie miało prefiks identyfikatora tenant'a. **/
    id: string
    /** Ostateczny tekst wzmianki @, łącznie z symbolem @. **/
    tag: string
    /** Oryginalny tekst wzmianki @, łącznie z symbolem @. **/
    rawTag: string
    /** Typ oznaczonego użytkownika. user = konto FastComments.com. sso = Użytkownik SSO. **/
    type: 'user'|'sso'
    /** Jeśli użytkownik zrezygnuje z powiadomień, to nadal będzie ustawione na true. **/
    sent: boolean
}
[inline-code-end]

### Hashtagi

When hashtags are used and successfully parsed, the information is stored in a list called `hashTags`. Each object in that list
has the following structure. Hashtags can also be manually added to the comment `hashTags` array for querying, if `retain` is set.

[inline-code-attrs-start title = 'Obiekt hashtagu komentarza'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** Id hashtagu. **/
    id: string
    /** Ostateczny tekst tagu #hashtag, łącznie ze symbolem #. **/
    tag: string
    /** Jeśli hashtag jest powiązany z niestandardowym URL, to będzie to zdefiniowane. **/
    url?: string
    /** Czy powinniśmy zachować hashtag, nawet jeśli nie istnieje w treści komentarza podczas aktualizacji. Przydatne do tagowania komentarzy bez zmiany treści komentarza. **/
    retain?: boolean
}
[inline-code-end]

---