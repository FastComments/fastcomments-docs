Die einzige Struktur, die über Webhooks gesendet wird, ist das WebhookComment-Objekt, unten in TypeScript dargestellt.

#### Die Struktur des WebhookComment-Objekts

##### Die Struktur des "create"-Events
Der Request-Body des "create"-Events ist ein WebhookComment-Objekt.

##### Die Struktur des "update"-Events
Der Request-Body des "update"-Events ist ein WebhookComment-Objekt.

##### Die Struktur des "delete"-Events
Der Request-Body des "delete"-Events ist ein WebhookComment-Objekt.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Das WebhookComment-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Die id des Kommentars. **/
    id: string
    /** Die id oder URL, die den Kommentarthread identifiziert. Normalisiert. **/
    urlId: string
    /** Die URL, die auf den Ort zeigt, an dem der Kommentar hinterlassen wurde. **/
    url?: string
    /** Die user id, der den Kommentar hinterlassen hat. Bei SSO mit tenant id vorangestellt. **/
    userId?: string
    /** Die E-Mail des Benutzers, der den Kommentar hinterlassen hat. **/
    commenterEmail?: string
    /** Der Name des Benutzers, der im Kommentar-Widget angezeigt wird. Bei SSO kann es displayName sein. **/
    commenterName: string
    /** Rohtext des Kommentars. **/
    comment: string
    /** Kommentartext nach dem Parsen. **/
    commentHTML: string
    /** Externe Kommentar-id. **/
    externalId?: string
    /** Die id des übergeordneten Kommentars. **/
    parentId?: string | null
    /** Das UTC-Datum, wann der Kommentar hinterlassen wurde. **/
    date: UTC_ISO_DateString
    /** Kombinierter Karma-Wert (up - down) der Votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True, wenn der Benutzer zum Zeitpunkt des Kommentierens eingeloggt war, oder er den Kommentar verifiziert hat, oder wenn er seine Sitzung beim Hinterlassen des Kommentars verifiziert hat. **/
    verified: boolean
    /** Datum, wann der Kommentar verifiziert wurde. **/
    verifiedDate?: number
    /** Ob ein Moderator den Kommentar als geprüft markiert hat. **/
    reviewed: boolean
    /** Der Speicherort bzw. die Base64-Codierung des Avatars. Wird nur base64 sein, wenn dieser Wert bei SSO übergeben wurde. **/
    avatarSrc?: string
    /** Wurde der Kommentar manuell oder automatisch als Spam markiert? **/
    isSpam: boolean
    /** Wurde der Kommentar automatisch als Spam markiert? **/
    aiDeterminedSpam: boolean
    /** Sind Bilder im Kommentar vorhanden? **/
    hasImages: boolean
    /** Die Seitenzahl, auf der sich der Kommentar für die Sortierung "Most Relevant" befindet. **/
    pageNumber: number
    /** Die Seitenzahl, auf der sich der Kommentar für die Sortierung "Oldest First" befindet. **/
    pageNumberOF: number
    /** Die Seitenzahl, auf der sich der Kommentar für die Sortierung "Newest First" befindet. **/
    pageNumberNF: number
    /** Wurde der Kommentar automatisch oder manuell genehmigt? **/
    approved: boolean
    /** Der Locale-Code (Format: en_us) des Benutzers zum Zeitpunkt des Abfassens des Kommentars. **/
    locale: string
    /** Die im Kommentar geschriebenen @mentions, die erfolgreich geparst wurden. **/
    mentions?: CommentUserMention[]
    /** Die Domain, von der der Kommentar stammt. **/
    domain?: string
    /** Die optionale Liste der mit diesem Kommentar verbundenen Moderationsgruppen-ids. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Wenn Benutzer in einem Kommentar markiert werden, werden die Informationen in einer Liste namens `mentions` gespeichert. Jedes Objekt in dieser Liste
hat die folgende Struktur.

[inline-code-attrs-start title = 'Das Webhook-Mentions-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Die user id. Bei SSO-Benutzern wird die tenant id vorangestellt. **/
    id: string
    /** Der finale @mention-Tagtext inklusive des @-Symbols. **/
    tag: string
    /** Der ursprüngliche @mention-Tagtext inklusive des @-Symbols. **/
    rawTag: string
    /** Welche Art von Benutzer markiert wurde. user = FastComments.com-Konto. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Wenn der Benutzer Benachrichtigungen abbestellt, wird dies trotzdem auf true gesetzt. **/
    sent: boolean
}
[inline-code-end]

#### Verwendete HTTP-Methoden

**Create und Update verwenden beide HTTP PUT und nicht POST!**

Da alle unsere Anfragen eine ID enthalten, sollte das Wiederholen derselben Create- oder Update-Anfrage auf Ihrer Seite keine neuen Objekte erstellen.

Das bedeutet, dass diese Aufrufe idempotent sind und gemäß der HTTP-Spezifikation als PUT-Ereignisse ausgeführt werden sollten.