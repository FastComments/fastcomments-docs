[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Erfasst eine Stimme zu einer Umfrage.

Ein Wähler hat höchstens eine Stimme pro Umfrage. Wenn diese Methode erneut für denselben Wähler aufgerufen wird, wird seine Stimme auf die neue Option verschoben, anstatt eine zweite hinzuzufügen, und das Abstimmen für die bereits gewählte Option bewirkt nichts.

Die Antwort enthält die Umfrage, sodass Sie die aktualisierten Zähler ohne eine zweite Anfrage erhalten.

[inline-code-attrs-start title = 'PollVote Erstellen cURL Beispiel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Anonymes PollVote Erstellen cURL Beispiel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Erstellen Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** One of userId or anonUserId is required. **/
    userId?: string
    anonUserId?: string
    /** The end user's IP, used for the anonymous rate limit. Defaults to the caller's IP. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Erstellen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Included on failure. **/
    reason?: string
    pollVote: PollVote
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Anonyme Stimmen

Setzen Sie `anonUserId` anstelle von `userId`, um eine Stimme für jemanden zu erfassen, der nicht eingeloggt ist. Diese ID muss nicht mit einem Benutzer irgendwo übereinstimmen – sie identifiziert lediglich die Sitzung, sodass dieselbe Person nicht doppelt gezählt wird.

Anonymes Abstimmen muss für Ihre Seite aktiviert sein. Wenn das Abstimmen auf eingeloggte Benutzer beschränkt ist, schlägt eine Stimme mit nur einem `anonUserId` mit `poll-login-required` fehl.

Anonyme Stimmen werden ebenfalls pro IP und pro Umfrage rate‑limitiert, um zu verhindern, dass eine Person eine Umfrage manipuliert, indem sie ihre Sitzung löscht. Senden Sie die IP des Endbenutzers (`ip`), damit das Limit für diesen und nicht für Ihren Server gilt.

### Weitere Hinweise

- Ein `userId` muss ein Benutzer sein, der auf Ihrer Seite existiert. Stimmen für einen Benutzer, der zu einer anderen Seite gehört, werden abgelehnt.
- Das Abstimmen in einer geschlossenen Umfrage schlägt mit `poll-closed` fehl.
- Diese API aktualisiert die Zähler der Umfrage und überträgt sie in Echtzeit an verbundene Widgets.