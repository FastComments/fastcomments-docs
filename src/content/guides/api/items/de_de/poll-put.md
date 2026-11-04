[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Fügt eine Umfrage an einen bestehenden Kommentar an oder setzt den vollständigen Zustand der Umfrage, die er bereits hat.

Der Body ist die vollständige Umfrage, und die Optionen, die Sie senden, werden in dieser Reihenfolge zu den Optionen der Umfrage. Jede Option
wird über ihre `id` abgeglichen:

- Eine Option, die mit der `id` einer bestehenden Option gesendet wird, behält diese Option und ihre Stimmen. Ihr Label und ihre Position
  werden auf die gesendeten Werte aktualisiert.
- Eine Option, die ohne `id` gesendet wird, wird ohne Stimmen hinzugefügt.
- Eine bestehende Option, die Sie weglassen, wird zusammen mit den auf sie abgegebenen Stimmen entfernt. `totalVotes` sinkt um denselben
  Betrag.

Um also eine Option hinzuzufügen, senden Sie die aktuellen Optionen mit ihren ids plus die neue ohne id. Um eine zu entfernen,
senden Sie die Liste ohne sie. Die Option-ids finden sich in der Umfrage, die von `GET /api/v1/polls/:commentId` zurückgegeben wird.

Wenn Sie gar keine ids senden, werden alle Optionen ersetzt und alle bereits auf die Umfrage abgegebenen Stimmen gelöscht. Wenn die Umfrage
Stimmen hat, ist dafür `replaceVotes=true` erforderlich; ohne diesen Parameter antwortet die API mit `replace-votes-required`.

Die anderen Felder werden ebenfalls ersetzt: Wenn Sie `closesAt`, `privacy` oder `requireVoteToSeeResults` weglassen, wird das jeweilige
Feld auf seinen Standardwert zurückgesetzt. Um ein einzelnes Feld zu ändern und den Rest unangetastet zu lassen, verwenden Sie `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Poll Put cURL-Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Put Anfrage-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Required to keep none of the existing option ids when the poll has votes, since that deletes them all. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** The id of an existing option, to keep it and its votes. Omit to add a new option. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** The complete, ordered list. Existing options left out are removed with their votes. **/
    options: PollPutOption[]
    /** Must be in the future when the comment has no poll yet. Omit for a poll that stays open. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Put Antwort-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Weitere Hinweise

- Eine `id`, die nicht auf der Umfrage vorhanden ist, oder dieselbe `id`, die zweimal angegeben wird, führt zu einem Fehler mit `poll-invalid`. Ein Kommentar ohne
  Umfrage hat noch keine Option-ids, daher muss bei jeder an ihn gesendeten Option `id` weggelassen werden.
- Die Privatsphäre-Einstellung einer Umfrage kann nach abgegebenen Stimmen eingeschränkt, aber nicht erweitert werden.
- Diese API folgt Ihren Seiteneinstellungen. Wenn Umfragen für die Seite oder die Seiten-URL nicht aktiviert sind, schlägt sie mit
  `polls-disabled` fehl.
- Ein gesperrter Kommentar kann nicht geändert werden und schlägt mit `locked` fehl.
- Verbundene Widgets werden live aktualisiert, sodass Betrachter die neue Umfrage ohne Neuladen sehen.