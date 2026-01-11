Die einzige Struktur, die über Webhooks gesendet wird, ist das WebhookComment-Objekt, unten in TypeScript dargestellt.

#### Die Struktur des WebhookComment-Objekts

##### Die "Create"-Event-Struktur
Der Request-Body des "create"-Events ist ein WebhookComment-Objekt.

##### Die "Update"-Event-Struktur
Der Request-Body des "update"-Events ist ein WebhookComment-Objekt.

##### Die "Delete"-Event-Struktur
Der Request-Body des "delete"-Events ist ein WebhookComment-Objekt.

    Änderung ab 14. Nov 2023
    Zuvor enthielt der Request-Body des "delete"-Events nur die Kommentar-ID. Er enthält jetzt den vollständigen Kommentar zum Zeitpunkt der Löschung.


[inline-code-attrs-start title = 'Das WebhookComment-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Die id des Kommentars. **/
    id: string
    /** Die id oder URL, die den Kommentar-Thread identifiziert. Normalisiert. **/
    urlId: string
    /** Die URL, die auf den Ort zeigt, an dem der Kommentar hinterlassen wurde. **/
    url?: string
    /** Die Benutzer-id, die den Kommentar hinterlassen hat. Bei SSO mit Tenant-id vorangestellt. **/
    userId?: string
    /** Die E-Mail des Benutzers, der den Kommentar hinterlassen hat. **/
    commenterEmail?: string
    /** Der Name des Benutzers, der im Kommentar-Widget angezeigt wird. Bei SSO kann dies displayName sein. **/
    commenterName: string
    /** Rohkommentartext. **/
    comment: string
    /** Kommentartext nach dem Parsen. **/
    commentHTML: string
    /** Externe Kommentar-id. **/
    externalId?: string
    /** Die id des übergeordneten Kommentars. **/
    parentId?: string | null
    /** Das UTC-Datum, an dem der Kommentar hinterlassen wurde. **/
    date: UTC_ISO_DateString
    /** Kombiniertes Karma (up - down) der Stimmen. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Wahr, wenn der Benutzer beim Kommentieren eingeloggt war, oder seinen Kommentar verifiziert hat, oder wenn er seine Sitzung beim Hinterlassen des Kommentars verifiziert hatte. **/
    verified: boolean
    /** Datum, wann der Kommentar verifiziert wurde. **/
    verifiedDate?: number
    /** Ob ein Moderator den Kommentar als überprüft markiert hat. **/
    reviewed: boolean
    /** Der Speicherort oder die Base64-Codierung des Avatars. Wird nur Base64 sein, wenn dieser Wert mit SSO übergeben wurde. **/
    avatarSrc?: string
    /** Wurde der Kommentar manuell oder automatisch als Spam markiert? **/
    isSpam: boolean
    /** Wurde der Kommentar automatisch als Spam markiert? **/
    aiDeterminedSpam: boolean
    /** Enthält der Kommentar Bilder? **/
    hasImages: boolean
    /** Die Seitenzahl, auf der sich der Kommentar bei der Sortierung "Most Relevant" befindet. **/
    pageNumber: number
    /** Die Seitenzahl, auf der sich der Kommentar bei der Sortierung "Oldest First" befindet. **/
    pageNumberOF: number
    /** Die Seitenzahl, auf der sich der Kommentar bei der Sortierung "Newest First" befindet. **/
    pageNumberNF: number
    /** Wurde der Kommentar automatisch oder manuell genehmigt? **/
    approved: boolean
    /** Der Locale-Code (Format: en_us) des Benutzers beim Schreiben des Kommentars. **/
    locale: string
    /** Die im Kommentar geschriebenen @mentions, die erfolgreich geparst wurden. **/
    mentions?: CommentUserMention[]
    /** Die Domain, von der der Kommentar stammt. **/
    domain?: string
    /** Die optionale Liste von Moderationsgruppen-ids, die mit diesem Kommentar verknüpft sind. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Das Webhook-Mentions-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Die Benutzer-id. Für SSO-Benutzer wird Ihre Tenant-id vorangestellt. **/
    id: string
    /** Der endgültige @mention-Tag-Text, einschließlich des @-Symbols. **/
    tag: string
    /** Der ursprüngliche @mention-Tag-Text, einschließlich des @-Symbols. **/
    rawTag: string
    /** Welcher Benutzertyp markiert wurde. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Wenn der Benutzer Benachrichtigungen abwählt, bleibt dieses Feld trotzdem auf true gesetzt. **/
    sent: boolean
}
[inline-code-end]

#### HTTP-Methoden

Sie können die HTTP-Methode für jeden Webhook-Ereignistyp im Admin-Panel konfigurieren:

- **Create Event**: POST oder PUT (Standard: PUT)
- **Update Event**: POST oder PUT (Standard: PUT)
- **Delete Event**: DELETE, POST oder PUT (Standard: DELETE)

Da alle Anfragen eine ID enthalten, sind Create- und Update-Operationen standardmäßig idempotent (PUT). Die wiederholte Zusendung derselben Create- oder Update-Anfrage sollte auf Ihrer Seite keine doppelten Objekte erzeugen.

#### Request-Header

Jede Webhook-Anfrage enthält folgende Header:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ihr API-Secret |
| `X-FastComments-Timestamp` | Unix-Zeitstempel (Sekunden), zu dem die Anfrage signiert wurde |
| `X-FastComments-Signature` | HMAC-SHA256-Signatur (`sha256=<hex>`) |

Siehe [Sicherheit & API-Token](/guides/webhooks/webhooks-api-tokens) für Informationen zur Überprüfung der HMAC-Signatur.

---