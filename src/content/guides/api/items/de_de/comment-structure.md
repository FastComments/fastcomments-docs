Ein `Comment`-Objekt stellt einen von einem Benutzer hinterlassenen Kommentar dar.

Die Beziehung zwischen Eltern- und Kindkommentaren wird über `parentId` definiert.

Die Struktur des `Comment`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'Kommentarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Auf true gesetzt, wenn die Spam-Engine festgestellt hat, dass der Kommentar Spam ist. **/
    aiDeterminedSpam?: boolean
    /** Ob der Kommentar zur Anzeige freigegeben ist. Wird beim Speichern des Kommentars auf true gesetzt, andernfalls wird er ausgeblendet. **/
    approved?: boolean
    /** Das Avatarbild des Benutzers. **/
    avatarSrc?: string
    /** Kinderkommentare. Nicht in allen Szenarien befüllt. Wird verwendet, wenn asTree über die API auf true gesetzt ist. **/
    children: Comment[]
    /** Der vom Kommentierenden eingegebene Rohkommentar. **/
    comment: string
    /** READONLY: Der in HTML geparste Kommentar des Kommentierenden. **/
    commentHTML?: string
    /** Die E-Mail des Kommentierenden. Erforderlich, wenn anonymes Kommentieren deaktiviert ist. **/
    commenterEmail?: string
    /** Der Link des Kommentierenden (z. B. sein Blog). **/
    commenterLink?: string
    /** Der Name des Kommentierenden. Immer erforderlich. Falls nicht verfügbar, auf etwas wie "Anonymous" setzen. **/
    commenterName: string
    /** Das Datum, an dem der Kommentar hinterlassen wurde, in UTC-Epoch. **/
    date: number
    /** Das "Anzeigelabel" für den Kommentar - zum Beispiel "Admin", "Moderator" oder etwas wie "VIP User". **/
    displayLabel?: string
    /** Die Domain, auf der der Kommentar gepostet wurde. **/
    domain?: string
    /** READONLY: Die Anzahl der Markierungen ("Flaggen") für den Kommentar. **/
    flagCount?: number
    /** Die #hashtags, die im Kommentar geschrieben und erfolgreich geparst wurden. Sie können Hashtags auch manuell hinzufügen, um Abfragen zu ermöglichen, sie werden jedoch nicht automatisch im Kommentartext angezeigt. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Enthält der Kommentar Bilder? **/
    hasImages?: boolean
    /** READONLY: Enthält der Kommentar Links? **/
    hasLinks?: boolean
    /** READONLY: Die eindeutige Kommentar-ID. **/
    id: string
    /** Nur beim Erstellen! Dies wird für die Speicherung gehasht. **/
    ip?: string
    /** READONLY: Hat der aktuelle Benutzer den Benutzer, der diesen Kommentar geschrieben hat, blockiert? **/
    isBlocked?: boolean
    /** READONLY: Ist der Kommentar von einem Admin? Wird automatisch basierend auf userId gesetzt. **/
    isByAdmin?: boolean
    /** READONLY: Ist der Kommentar von einem Moderator? Wird automatisch basierend auf userId gesetzt. **/
    isByModerator?: boolean
    /** Auf true gesetzt, wenn der Kommentar soft gelöscht wurde (ein Platzhalter musste aufgrund einer anderen Konfiguration belassen werden). **/
    isDeleted?: boolean
    /** Auf true gesetzt, wenn das Benutzerkonto gelöscht wurde und der Kommentar erhalten bleiben musste. **/
    isDeletedUser?: boolean
    /** READONLY: Wurde der Kommentar vom aktuell angemeldeten Benutzer (contextUserId) markiert? **/
    isFlagged?: boolean
    /** Ist der Kommentar angepinnt? **/
    isPinned?: boolean
    /** Ist der Kommentar gesperrt? Wenn true, kann niemand (einschließlich Moderatoren) darauf antworten, ihn bearbeiten oder löschen, bis er entsperrt wird. **/
    isLocked?: boolean
    /** Ist der Kommentar Spam? **/
    isSpam?: boolean
    /** READONLY: Wurde der Kommentar vom aktuellen Benutzer (contextUserId) nach unten bewertet? **/
    isVotedDown?: boolean
    /** READONLY: Wurde der Kommentar vom aktuellen Benutzer (contextUserId) nach oben bewertet? **/
    isVotedUp?: boolean
    /** Die Locale, in der sich der Kommentar befindet. Falls nicht angegeben, wird sie aus dem Language-Accept-HTTP-Header abgeleitet. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: Die in dem Kommentar erfolgreich geparsten @mentions. **/
    mentions?: CommentUserMention[]
    /** Optionale Metadaten, die dem Kommentar zugeordnet sind. **/
    meta?: Record<string, string | number | boolean>
    /** Die optionale Liste der Moderationsgruppen-IDs, die mit diesem Kommentar verknüpft sind. **/
    moderationGroupIds?: string[]|null
    /** READONLY: Die ID des Vote-Objekts, das der Abstimmung des aktuellen Benutzers (contextUserId) zu diesem Kommentar entspricht. **/
    myVoteId?: string
    /** Ob für diesen Kommentar Benachrichtigungen an Kommentierende gesendet wurden. Um zu verhindern, dass bei Imports Benachrichtigungen gesendet werden, setzen Sie dies auf true. **/
    notificationSentForParent?: boolean
    /** Ob für diesen Kommentar Benachrichtigungen an Mandantenbenutzer gesendet wurden. Um zu verhindern, dass bei Imports Benachrichtigungen gesendet werden, setzen Sie dies auf true. **/
    notificationSentForParentTenant?: boolean
    /** Der Titel der Seite, auf der sich dieser Kommentar befand. **/
    pageTitle?: string
    /** Wenn wir auf einen Kommentar antworten, ist dies die ID, auf die wir antworten. **/
    parentId?: string|null
    /** Ob der Kommentar als geprüft markiert ist. **/
    reviewed: boolean
    /** Die Mandanten-ID, zu der der Kommentar gehört. **/
    tenantId: string
    /** Der Benutzer, der den Kommentar geschrieben hat. Wird beim Speichern eines Kommentars mit Name/E-Mail automatisch erstellt. **/
    userId?: string|null
    /** Die URL zu dem Ort, an dem dieser Kommentar sichtbar ist, z. B. ein Blogbeitrag. **/
    url: string
    /** Eine "bereinigte" Version der von Ihnen übergebenen urlId. Beim Speichern geben Sie dieses Feld an, beim Abrufen wird es jedoch "bereinigt" und Ihr Originalwert in "urlIdRaw" verschoben. **/
    urlId: string
    /** READONLY: Die originale urlId, die Sie übergeben haben. **/
    urlIdRaw?: string
    /** Ist der Benutzer und dieser Kommentar verifiziert? **/
    verified: boolean
    /** Anzahl der Upvotes. **/
    votesUp?: number
    /** Anzahl der Downvotes. **/
    votesDown?: number
    /** Das "Karma" des Kommentars (= Upvotes - Downvotes). **/
    votes?: number
}
[inline-code-end]

Einige dieser Felder sind mit `READONLY` gekennzeichnet – diese werden von der API zurückgegeben, können jedoch nicht gesetzt werden.

### Kommentar-Textstruktur

Kommentare werden in einer FastComments-Variante von Markdown geschrieben, die einfach Markdown plus traditionelle `bbcode`-artige Tags für Bilder ist, wie `[img]path[/img]`.

Text wird in zwei Feldern gespeichert. Der vom Benutzer eingegebene Text wird unverändert im Feld `comment` gespeichert. Dieser wird gerendert und im Feld `commentHTML` gespeichert.

Die erlaubten HTML-Tags sind `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Es wird empfohlen, das HTML zu rendern, da es sich nur um eine sehr kleine Teilmenge von HTML handelt; einen Renderer zu bauen ist daher ziemlich unkompliziert. Es gibt beispielsweise mehrere Bibliotheken für React Native und Flutter, die dabei helfen.

Sie können sich entscheiden, den nicht normalisierten Wert des `comment`-Felds zu rendern. [Ein Beispielparser ist hier.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Der Beispielparser könnte auch angepasst werden, um mit HTML zu arbeiten und die HTML-Tags in die erwarteten Elemente zu transformieren, die für Ihre Plattform gerendert werden sollen. 

### Markierungen

Wenn Benutzer in einem Kommentar markiert werden, werden die Informationen in einer Liste namens `mentions` gespeichert. Jedes Objekt in dieser Liste
hat die folgende Struktur.

[inline-code-attrs-start title = 'Das Kommentar-Erwähnungsobjekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Die Benutzer-ID. Bei SSO-Benutzern ist Ihre Mandanten-ID vorangestellt. **/
    id: string
    /** Der finale @mention-Tagtext, einschließlich des @-Symbols. **/
    tag: string
    /** Der ursprüngliche @mention-Tagtext, einschließlich des @-Symbols. **/
    rawTag: string
    /** Welche Art von Benutzer markiert wurde. user = FastComments.com-Konto. sso = SSO-Benutzer. **/
    type: 'user'|'sso'
    /** Wenn der Benutzer Benachrichtigungen deaktiviert, bleibt dies trotzdem auf true gesetzt. **/
    sent: boolean
}
[inline-code-end]

### Hashtags

Wenn Hashtags verwendet und erfolgreich geparst werden, werden die Informationen in einer Liste namens `hashTags` gespeichert. Jedes Objekt in dieser Liste
hat die folgende Struktur. Hashtags können auch manuell dem `hashTags`-Array des Kommentars für Abfragen hinzugefügt werden, wenn `retain` gesetzt ist.

[inline-code-attrs-start title = 'Das Hashtag-Objekt des Kommentars'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** Die Hashtag-ID. **/
    id: string
    /** Der finale #hashtag-Tagtext, einschließlich des #-Symbols. **/
    tag: string
    /** Wenn das Hashtag mit einer benutzerdefinierten URL verknüpft ist, ist diese definiert. **/
    url?: string
    /** Ob wir das Hashtag beibehalten sollen, selbst wenn es beim Aktualisieren des Kommentars nicht im Kommentartext vorhanden ist. Nützlich, um Kommentare zu taggen, ohne den Kommentartext zu ändern. **/
    retain?: boolean
}
[inline-code-end]

---