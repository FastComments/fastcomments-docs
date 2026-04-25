Obiekt `Comment` reprezentuje komentarz pozostawiony przez użytkownika.

Relacja między komentarzami rodzica i potomka jest określana za pomocą `parentId`.

Struktura obiektu Comment jest następująca:

[inline-code-attrs-start title = 'Struktura obiektu Comment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Ustaw na true, jeśli silnik antyspamowy uznał komentarz za spam. **/
    aiDeterminedSpam?: boolean
    /** Czy komentarz jest zatwierdzony do wyświetlania. Ustaw na true przy zapisie komentarza, w przeciwnym razie będzie ukryty. **/
    approved?: boolean
    /** Avatar użytkownika. **/
    avatarSrc?: string
    /** Komentarze potomne. Nie zawsze wypełnione. Używane, gdy parametr asTree jest ustawiony na true przez API. **/
    children: Comment[]
    /** Surowy tekst komentarza. **/
    comment: string
    /** READONLY: Komentarz autora sparsowany do HTML. **/
    commentHTML?: string
    /** Email autora komentarza. Wymagany, jeśli anonimowe komentowanie jest wyłączone. **/
    commenterEmail?: string
    /** Link autora komentarza (np. jego blog). **/
    commenterLink?: string
    /** Nazwa autora komentarza. Zawsze wymagana. Jeśli niedostępna, ustaw np. "Anonim". **/
    commenterName: string
    /** Data pozostawienia komentarza, w formacie epoch UTC. **/
    date: number
    /** „Etykieta wyświetlana” dla komentarza — na przykład "Admin", "Moderator" lub coś w rodzaju "Użytkownik VIP". **/
    displayLabel?: string
    /** Domenę, na której opublikowano komentarz. **/
    domain?: string
    /** READONLY: Liczba razy, kiedy komentarz został zgłoszony. **/
    flagCount?: number
    /** #hashtagi zapisane w komentarzu, które zostały poprawnie sparsowane. Można również ręcznie dodać hashtagi do zapytań, ale nie będą one automatycznie wyświetlane w treści komentarza. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Czy komentarz zawiera obrazy? **/
    hasImages?: boolean
    /** READONLY: Czy komentarz zawiera linki? **/
    hasLinks?: boolean
    /** READONLY: Unikalne id komentarza. **/
    id: string
    /** Tylko podczas tworzenia! To jest haszowane do przechowywania. **/
    ip?: string
    /** READONLY: Czy bieżący użytkownik zablokował autora tego komentarza? **/
    isBlocked?: boolean
    /** READONLY: Czy komentarz został napisany przez administratora? Ustawiane automatycznie na podstawie userId. **/
    isByAdmin?: boolean
    /** READONLY: Czy komentarz został napisany przez moderatora? Ustawiane automatycznie na podstawie userId. **/
    isByModerator?: boolean
    /** Ustaw na true, jeśli komentarz został soft usunięty (zostawiono zastępczy wpis z powodu innej konfiguracji). **/
    isDeleted?: boolean
    /** Ustaw na true, jeśli konto użytkownika zostało usunięte, a komentarz musiał zostać zachowany. **/
    isDeletedUser?: boolean
    /** READONLY: Czy komentarz został oznaczony (flag) przez aktualnie zalogowanego użytkownika (contextUserId)? **/
    isFlagged?: boolean
    /** Czy komentarz jest przypięty? **/
    isPinned?: boolean
    /** Czy komentarz jest zablokowany? Gdy true, nikt (włącznie z moderatorami) nie może na niego odpowiadać, edytować ani usuwać go dopóki nie zostanie odblokowany. **/
    isLocked?: boolean
    /** Czy komentarz to spam? **/
    isSpam?: boolean
    /** READONLY: Czy komentarz został oceniony negatywnie przez bieżącego użytkownika (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: Czy komentarz został oceniony pozytywnie przez bieżącego użytkownika (contextUserId)? **/
    isVotedUp?: boolean
    /** Lokalizacja (locale), w której jest komentarz. Jeśli nie podano, zostanie wyprowadzona z nagłówka Accept-Language HTTP. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: @wzmianki zapisane w komentarzu, które zostały poprawnie sparsowane. **/
    mentions?: CommentUserMention[]
    /** Opcjonalne metadane związane z komentarzem. **/
    meta?: Record<string, string | number | boolean>
    /** Opcjonalna lista id grup moderacji powiązanych z tym komentarzem. **/
    moderationGroupIds?: string[]|null
    /** READONLY: Id obiektu głosu odpowiadającego głosowi bieżącego użytkownika (contextUserId) na ten komentarz. **/
    myVoteId?: string
    /** Czy powiadomienia zostały wysłane dla komentujących w związku z tym komentarzem. Aby zapobiec wysyłaniu powiadomień przy imporcie, ustaw to na true. **/
    notificationSentForParent?: boolean
    /** Czy powiadomienia zostały wysłane dla użytkowników najemcy (tenant) w związku z tym komentarzem. Aby zapobiec wysyłaniu powiadomień przy imporcie, ustaw to na true. **/
    notificationSentForParentTenant?: boolean
    /** Tytuł strony, na której znajdował się ten komentarz. **/
    pageTitle?: string
    /** Jeśli odpowiadamy na komentarz, to jest ID, na który odpowiadamy. **/
    parentId?: string|null
    /** Czy komentarz jest oznaczony jako sprawdzony (reviewed). **/
    reviewed: boolean
    /** Id tenant-a (najemcy), do którego należy komentarz. **/
    tenantId: string
    /** Użytkownik, który napisał komentarz. Tworzony automatycznie przy zapisie komentarza z nazwą/email. **/
    userId?: string|null
    /** URL miejsca, w którym widoczny jest ten komentarz, np. wpis na blogu. **/
    url: string
    /** „Oczyszczona” wersja urlId, które przesłano. Przy zapisie podajesz to pole, ale po pobraniu komentarza zostanie ono „oczyszczone”, a Twoja oryginalna wartość przeniesiona do urlIdRaw. **/
    urlId: string
    /** READONLY: Oryginalne urlId, które nam przesłałeś. **/
    urlIdRaw?: string
    /** Czy użytkownik i ten komentarz są zweryfikowani? **/
    verified: boolean
    /** Liczba głosów za. **/
    votesUp?: number
    /** Liczba głosów przeciw. **/
    votesDown?: number
    /** „Karma” komentarza (= głosy za - głosy przeciw). **/
    votes?: number
}
[inline-code-end]

Niektóre z tych pól są oznaczone jako `READONLY` — są zwracane przez API, ale nie można ich ustawić.

### Struktura tekstu komentarza

Komentarze są pisane w odmianie markdown FastComments, która jest zwykłym markdownem z dodatkowymi, tradycyjnymi tagami w stylu `bbcode` dla obrazów, np. `[img]path[/img]`.

Tekst jest przechowywany w dwóch polach. Tekst wpisany przez użytkownika jest przechowywany bez zmian w polu `comment`. Ten tekst jest renderowany i zapisywany w polu `commentHTML`.

Dozwolone tagi HTML to `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li i br`.

Zaleca się renderowanie HTML, ponieważ jest to bardzo mały podzbiór HTML, więc zbudowanie renderera jest dość proste. Istnieje kilka bibliotek, np. dla React Native i Flutter, które w tym pomagają

Możesz wybrać renderowanie nienormalizowanej wartości pola `comment`. [Przykładowy parser znajduje się tutaj.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Przykładowy parser można również dostosować do pracy z HTML i przekształcania tagów HTML w oczekiwane elementy do renderowania na Twojej platformie. 

### Tagging

Gdy użytkownicy są oznaczani w komentarzu, informacje są przechowywane na liście zwanej `mentions`. Każdy obiekt na tej liście
ma następującą strukturę.

[inline-code-attrs-start title = 'Obiekt wzmianki w komentarzu'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id użytkownika. Dla użytkowników SSO będzie poprzedzone id tenant-a. **/
    id: string
    /** Końcowy tekst @wzmianki, łącznie z symbolem @. **/
    tag: string
    /** Oryginalny tekst @wzmianki, łącznie z symbolem @. **/
    rawTag: string
    /** Jaki typ użytkownika został oznaczony. user = konto FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Jeśli użytkownik rezygnuje z powiadomień, to nadal zostanie ustawione na true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Gdy hashtagi są używane i poprawnie sparsowane, informacje są przechowywane na liście zwanej `hashTags`. Każdy obiekt na tej liście
ma następującą strukturę. Hashtagi można również ręcznie dodać do tablicy `hashTags` komentarza do celów zapytań, jeśli ustawiono `retain`.

[inline-code-attrs-start title = 'Obiekt hashtagu komentarza'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** Id hashtagu. **/
    id: string
    /** Końcowy tekst #hashtagu, łącznie ze symbolem #. **/
    tag: string
    /** Jeśli hashtag jest powiązany z niestandardowym URL, będzie to zdefiniowane. **/
    url?: string
    /** Czy powinniśmy zachować hashtag, nawet jeśli nie istnieje w treści komentarza, gdy komentarz jest aktualizowany. Przydatne do tagowania komentarzy bez zmiany tekstu komentarza. **/
    retain?: boolean
}
[inline-code-end]