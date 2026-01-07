[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, Kommentare zu erstellen.

Häufige Anwendungsfälle sind benutzerdefinierte UIs, Integrationen oder Importe.

Hinweise:

- Diese API kann das Kommentar-Widget bei Bedarf "live" aktualisieren (dies erhöht `creditsCost` von `1` auf `2`).
- Diese API erstellt automatisch Benutzerobjekte in unserem System, wenn eine E-Mail angegeben wird.
- Der Versuch, zwei Kommentare mit unterschiedlichen E-Mails, aber demselben Benutzernamen zu speichern, führt zu einem Fehler für den zweiten Kommentar.
- Wenn Sie `parentId` angeben und ein untergeordneter Kommentar `notificationSentForParent` auf false hat, **werden wir Benachrichtigungen für den übergeordneten Kommentar senden**. Dies geschieht stündlich (wir fassen die Benachrichtigungen zusammen, um die Anzahl der gesendeten E-Mails zu reduzieren).
- Wenn Sie Willkommens-E-Mails beim Erstellen von Benutzern oder Kommentarverifizierungs-E-Mails senden möchten, setzen Sie `sendEmails` in den Query-Parametern auf `true`.
- Über diese API erstellte Kommentare erscheinen auf den Analytics- und Moderationsseiten der Admin-App.
- "Schlechte Wörter" werden weiterhin in den Kommentatornamen und im Kommentartext maskiert, wenn die Einstellung aktiviert ist.
- Über diese API erstellte Kommentare können bei Bedarf immer noch auf Spam überprüft werden.
- Konfigurationen wie die maximale Kommentarlänge, wenn über die Customization Rule Admin-Seite konfiguriert, gelten auch hier.

Die minimal erforderlichen Daten zur Übermittlung, die im Kommentar-Widget angezeigt werden, sind wie folgt:

[inline-code-attrs-start title = 'Minimales Comment POST cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Eine realistischere Anfrage könnte so aussehen:

[inline-code-attrs-start title = 'Comment POST cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Comment POST Anfrage-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment POST Antwort-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Included on failure. **/
    reason?: string
    /** The created comment. **/
    comment?: Comment
    /** The associated user, which may or may not have already existed. **/
    user?: User
}
[inline-code-end]
