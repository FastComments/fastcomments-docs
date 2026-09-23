[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Bearbeitet eine Umfrage, ohne deren Stimmen zu verändern. Verwenden Sie dies, um einen Tippfehler in der Frage oder einer Option zu korrigieren, die Umfrage zu schließen oder wieder zu öffnen oder zu ändern, wer sehen darf, wer abgestimmt hat.

Optionen werden über ihre `id` adressiert, und ein `PATCH` benennt die von Ihnen angegebenen um. Um Optionen hinzuzufügen, zu entfernen oder neu zu ordnen, senden Sie die vollständige Optionsliste an `PUT /api/v1/polls/:commentId`: Optionen, die Sie mit ihren IDs senden, behalten dort ebenfalls ihre Stimmen.

Jedes Feld ist optional, aber es muss mindestens eines angegeben werden.

[inline-code-attrs-start title = 'Poll Patch cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Umfrage jetzt schließen cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Weitere Hinweise

- Das Benennen einer Options-ID, die nicht in der Umfrage vorhanden ist, schlägt mit `poll-invalid` fehl, anstatt stillschweigend nichts zu tun.
- Bezeichnungen müssen innerhalb der Umfrage eindeutig bleiben, wobei die Optionen, die Sie nicht ändern, mitgezählt werden.
- Im Gegensatz zum Erstellen einer Umfrage kann `closesAt` hier in der Vergangenheit liegen – das ist die Methode, um eine Umfrage sofort zu schließen.
- Die Umfrage-Privatsphäre kann eingeschränkt, aber nicht erweitert werden, sobald Stimmen vorhanden sind.
- Ein gesperrter Kommentar kann seine Umfrage nicht ändern und schlägt mit `locked` fehl.