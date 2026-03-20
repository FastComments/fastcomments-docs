Die einzige Struktur, die per Webhooks gesendet wird, ist das WebhookComment-Objekt, unten in TypeScript dargestellt.

#### Die Struktur des WebhookComment-Objekts

##### Die "Create"-Ereignisstruktur
Der "create"-Ereignis-Request-Body ist ein WebhookComment-Objekt.

##### Die "Update"-Ereignisstruktur
Der "update"-Ereignis-Request-Body ist ein WebhookComment-Objekt.

##### Die "Delete"-Ereignisstruktur
Der "delete"-Ereignis-Request-Body ist ein WebhookComment-Objekt.

    Änderung ab 14. Nov. 2023
    Zuvor enthielt der Request-Body des "delete"-Ereignisses nur die Kommentar-ID. Jetzt enthält er den vollständigen Kommentar zum Zeitpunkt der Löschung.


[inline-code-attrs-start title = 'Das WebhookComment-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Die id des Kommentars. **/
    id: string
    /** Die id oder URL, die den Kommentar-Thread identifiziert. Normalisiert. **/
    urlId: string
    /** Die URL, die auf die Stelle verweist, an der der Kommentar hinterlassen wurde. **/
    url?: string
    /** Die Nutzer-id, die den Kommentar hinterlassen hat. Bei SSO mit tenant id vorangestellt. **/
    userId?: string
    /** Die E-Mail des Nutzers, der den Kommentar hinterlassen hat. **/
    commenterEmail?: string
    /** Der Name des Nutzers, der im Kommentar-Widget angezeigt wird. Bei SSO kann es displayName sein. **/
    commenterName: string
    /** Rohkommentartext. **/
    comment: string
    /** Kommentartext nach dem Parsen. **/
    commentHTML: string
    /** Externe id des Kommentars. **/
    externalId?: string
    /** Die id des übergeordneten Kommentars. **/
    parentId?: string | null
    /** Das UTC-Datum, an dem der Kommentar hinterlassen wurde. **/
    date: UTC_ISO_DateString
    /** Kombinierter Karma-Wert (up - down) der Stimmen. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Wahr, wenn der Nutzer beim Hinterlassen des Kommentars eingeloggt war, oder den Kommentar verifiziert hat, oder seine Sitzung verifiziert hatte. **/
    verified: boolean
    /** Datum, wann der Kommentar verifiziert wurde. **/
    verifiedDate?: number
    /** Ob ein Moderator den Kommentar als geprüft markiert hat. **/
    reviewed: boolean
    /** Der Ort oder die base64-Codierung des Avatars. Wird nur base64 sein, wenn dieser Wert mit SSO übergeben wurde. **/
    avatarSrc?: string
    /** Wurde der Kommentar manuell oder automatisch als Spam markiert? **/
    isSpam: boolean
    /** Wurde der Kommentar automatisch als Spam markiert? **/
    aiDeterminedSpam: boolean
    /** Gibt es Bilder im Kommentar? **/
    hasImages: boolean
    /** Die Seitenzahl, auf der sich der Kommentar bei der Sortierreihenfolge "Most Relevant" befindet. **/
    pageNumber: number
    /** Die Seitenzahl, auf der sich der Kommentar bei der Sortierreihenfolge "Oldest First" befindet. **/
    pageNumberOF: number
    /** Die Seitenzahl, auf der sich der Kommentar bei der Sortierreihenfolge "Newest First" befindet. **/
    pageNumberNF: number
    /** Wurde der Kommentar automatisch oder manuell genehmigt? **/
    approved: boolean
    /** Der Locale-Code (Format: en_us) des Nutzers, als der Kommentar geschrieben wurde. **/
    locale: string
    /** Die @mentions, die im Kommentar geschrieben und erfolgreich geparst wurden. **/
    mentions?: CommentUserMention[]
    /** Die Domain, aus der der Kommentar stammt. **/
    domain?: string
    /** Die optionale Liste von Moderationsgruppen-ids, die mit diesem Kommentar verknüpft sind. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Das Webhook Mentions-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Die nutzer-id. Bei SSO-Benutzern wird Ihre tenant id vorangestellt. **/
    id: string
    /** Der finale @mention-Tag-Text, inklusive des @-Symbols. **/
    tag: string
    /** Der ursprüngliche @mention-Tag-Text, inklusive des @-Symbols. **/
    rawTag: string
    /** Welche Art von Benutzer getaggt wurde. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Wenn der Benutzer Benachrichtigungen abbestellt, wird dies trotzdem auf true gesetzt. **/
    sent: boolean
}
[inline-code-end]

#### HTTP-Methoden

Sie können die HTTP-Methode für jeden Webhook-Ereignistyp im Admin-Panel konfigurieren:

- **Create-Ereignis**: POST oder PUT (Standard: PUT)
- **Update-Ereignis**: POST oder PUT (Standard: PUT)
- **Delete-Ereignis**: DELETE, POST oder PUT (Standard: DELETE)

Da alle Requests eine ID enthalten, sind Create- und Update-Operationen standardmäßig idempotent (PUT). Die Wiederholung derselben Create- oder Update-Anfrage sollte auf Ihrer Seite keine doppelten Objekte erzeugen.

#### Request-Header

Jede Webhook-Anfrage enthält die folgenden Header:

| Header | Beschreibung |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ihr API Secret |
| `X-FastComments-Timestamp` | Unix-Zeitstempel (Sekunden), als die Anfrage signiert wurde |
| `X-FastComments-Signature` | HMAC-SHA256-Signatur (`sha256=<hex>`) |

Siehe [Sicherheit & API-Tokens](/guide-webhooks.html#webhooks-api-tokens) für Informationen zur Verifizierung der HMAC-Signatur.

---