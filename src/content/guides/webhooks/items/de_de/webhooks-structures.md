Die einzige Struktur, die über Webhooks gesendet wird, ist das WebhookComment-Objekt, das unten in TypeScript dargestellt ist.

#### Die Struktur des WebhookComment-Objekts

##### Die Struktur des „Create“-Ereignisses
Der Request-Body des „create“-Ereignisses ist ein WebhookComment-Objekt.

##### Die Struktur des „Update“-Ereignisses
Der Request-Body des „update“-Ereignisses ist ein WebhookComment-Objekt.

##### Die Struktur des „Delete“-Ereignisses
Der Request-Body des „delete“-Ereignisses ist ein WebhookComment-Objekt.

    Änderung vom 14. November 2023
    Zuvor enthielt der Request-Body des „delete“-Ereignisses nur die Kommentar-ID. Er enthält jetzt den vollständigen Kommentar zum Zeitpunkt der Löschung.


Jeder Schlüssel ist im Body immer vorhanden. Wenn der Kommentar für ein Feld keinen Wert hat, trägt der Body `null`
(oder `false` für Booleans und `[]` für Listen), sodass die Form einer Zustellung von Kommentar zu Kommentar nie variiert.

[inline-code-attrs-start title = 'Das WebhookComment-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Die ID des Kommentars. **/
    id: string
    /** Die ID oder URL, die den Kommentar-Thread identifiziert. Normalisiert. **/
    urlId: string
    /** Die URL, die auf den Ort verweist, an dem der Kommentar hinterlassen wurde. **/
    url: string | null
    /** Die Benutzer-ID, die den Kommentar hinterlassen hat. Bei SSO mit der Tenant-ID vorangestellt. **/
    userId: string | null
    /** Die E-Mail des Benutzers, der den Kommentar hinterlassen hat. **/
    commenterEmail: string | null
    /** Der Name des Benutzers, der im Kommentar-Widget angezeigt wird. Bei SSO kann dies displayName sein. **/
    commenterName: string
    /** Roher Kommentartext. **/
    comment: string
    /** Kommentartext nach dem Parsen. **/
    commentHTML: string
    /** Externe ID des Kommentars. **/
    externalId: string | null
    /** Die ID des übergeordneten Kommentars. **/
    parentId: string | null
    /** Das UTC-Datum, an dem der Kommentar hinterlassen wurde. **/
    date: UTC_ISO_DateString
    /** Kombiniertes Karma (up - down) der Stimmen. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True, wenn der Benutzer beim Kommentieren eingeloggt war, oder den Kommentar verifiziert hat, oder seine Sitzung verifiziert hat, als der Kommentar hinterlassen wurde. **/
    verified: boolean
    /** Das UTC-Datum, an dem der Kommentar verifiziert wurde. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Ob ein Moderator den Kommentar als geprüft markiert hat. **/
    reviewed: boolean
    /** Der Speicherort oder die Base64-Kodierung des Avatars. Nur dann Base64, wenn dies der mit SSO übergebene Wert war. **/
    avatarSrc: string | null
    /** Wurde der Kommentar manuell oder automatisch als Spam markiert? **/
    isSpam: boolean
    /** Wurde der Kommentar automatisch als Spam markiert? **/
    aiDeterminedSpam: boolean
    /** Enthält der Kommentar Bilder? **/
    hasImages: boolean
    /** Die Seitenzahl, auf der sich der Kommentar für die Sortierrichtung „Most Relevant“ befindet. **/
    pageNumber: number | null
    /** Die Seitenzahl, auf der sich der Kommentar für die Sortierrichtung „Oldest First“ befindet. **/
    pageNumberOF: number | null
    /** Die Seitenzahl, auf der sich der Kommentar für die Sortierrichtung „Newest First“ befindet. **/
    pageNumberNF: number | null
    /** Wurde der Kommentar automatisch oder manuell genehmigt? **/
    approved: boolean
    /** Der Locale-Code (Format: en_us) des Benutzers, als der Kommentar geschrieben wurde. **/
    locale: string | null
    /** Die im Kommentar geschriebenen @mentions, die erfolgreich geparst wurden. Leer, wenn keine vorhanden sind. **/
    mentions: CommentUserMention[]
    /** Die Domain, von der der Kommentar stammt. **/
    domain: string | null
    /** Die mit diesem Kommentar verknüpften Moderationsgruppen-IDs. Leer, wenn keine vorhanden sind. **/
    moderationGroupIds: string[]
    /** Die Badges, die neben dem Namen des Kommentierenden bei diesem Kommentar angezeigt werden, zum Zeitpunkt der Erstellung des Ereignisses. Leer, wenn keine vorhanden sind. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Wenn Benutzer in einem Kommentar markiert werden, werden die Informationen in einer Liste namens `mentions` gespeichert. Jedes Objekt in dieser Liste
hat die folgende Struktur.

[inline-code-attrs-start title = 'Das Webhook-Mentions-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Die Benutzer-ID. Bei SSO-Benutzern wird Ihre Tenant-ID vorangestellt. **/
    id: string
    /** Der endgültige @mention-Tag-Text, einschließlich des @-Symbols. **/
    tag: string
    /** Der ursprüngliche @mention-Tag-Text, einschließlich des @-Symbols. **/
    rawTag: string
    /** Welche Art von Benutzer markiert wurde. user = FastComments.com-Konto. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Wenn der Benutzer Benachrichtigungen abbestellt hat, wird dies dennoch auf true gesetzt. **/
    sent: boolean
}
[inline-code-end]

Jeder Eintrag in der `badges`-Liste hat die folgende Struktur. Siehe den [Badges-Leitfaden](/guide-badges.html) für die Badge-Typen.

[inline-code-attrs-start title = 'Das Webhook-Badge-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** Die ID des diesem Benutzer verliehenen Badges. **/
    id: string
    /** Der Badge-Typ. **/
    type: number
    /** Die Badge-Beschreibung, die beim Daraufzeigen mit der Maus angezeigt wird. **/
    description: string
    /** Der auf dem Badge angezeigte Text. **/
    displayLabel?: string | null
    /** Das für den Badge angezeigte Bild, falls er eines verwendet. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Eine zusätzliche CSS-Klasse, die auf den Badge angewendet wird. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP-Methoden

Sie können die HTTP-Methode für jeden Webhook-Ereignistyp im Admin-Panel konfigurieren:

- **Create-Ereignis**: POST oder PUT (Standard: PUT)
- **Update-Ereignis**: POST oder PUT (Standard: PUT)
- **Delete-Ereignis**: DELETE, POST oder PUT (Standard: DELETE)

Da alle Anfragen eine ID enthalten, sind Create- und Update-Operationen standardmäßig idempotent (PUT). Das Wiederholen derselben Create- oder Update-Anfrage sollte auf Ihrer Seite keine doppelten Objekte erzeugen.

#### Request-Header

Jede Webhook-Anfrage enthält die folgenden Header:

| Header | Beschreibung |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ihr API-Secret |
| `X-FastComments-Timestamp` | Unix-Zeitstempel (Sekunden), wann die Anfrage signiert wurde |
| `X-FastComments-Signature` | HMAC-SHA256-Signatur (`sha256=<hex>`) |

Siehe [Sicherheit & API-Tokens](/guide-webhooks.html#webhooks-api-tokens) für Informationen zur Verifizierung der HMAC-Signatur.