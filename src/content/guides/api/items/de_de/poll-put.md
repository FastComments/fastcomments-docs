[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Fügt einer bestehenden Kommentar eine Umfrage hinzu oder setzt den vollständigen Zustand der bereits vorhandenen Umfrage.

Der Body ist die komplette Umfrage, und die Optionen, die Sie senden, werden zu den Umfrageoptionen, in dieser Reihenfolge. Jede Option wird anhand ihrer `id` zugeordnet:

- Eine Option, die mit der `id` einer bestehenden Option gesendet wird, behält diese Option und deren Stimmen. Ihre Bezeichnung und Position werden auf das von Ihnen gesendete aktualisiert.
- Eine Option, die ohne `id` gesendet wird, wird hinzugefügt, ohne Stimmen.
- Eine bestehende Option, die Sie weglassen, wird entfernt, zusammen mit den darauf abgegebenen Stimmen. `totalVotes` sinkt um denselben Betrag.

Um also eine Option hinzuzufügen, senden Sie die aktuellen Optionen mit ihren IDs plus die neue ohne ID. Um eine zu entfernen, senden Sie die Liste ohne sie. Die Options-IDs finden Sie in der Umfrage, die von `GET /api/v1/polls/:commentId` zurückgegeben wird.

Wenn Sie überhaupt keine IDs senden, werden alle Optionen ersetzt und jede bereits abgegebene Stimme in der Umfrage gelöscht. Hat die Umfrage Stimmen, erfordert dies `replaceVotes=true`; ohne diese Angabe antwortet die API mit `replace-votes-required`.

Die anderen Felder werden ebenfalls ersetzt: Wenn Sie `closesAt`, `privacy` oder `requireVoteToSeeResults` weglassen, wird der Standardwert wiederhergestellt. Um ein einzelnes Feld zu ändern und die anderen unverändert zu lassen, verwenden Sie `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Poll Put cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Other Notes

- Eine `id`, die nicht in der Umfrage vorhanden ist, oder dieselbe `id` zweimal angegeben, schlägt mit `poll-invalid` fehl. Ein Kommentar ohne Umfrage hat noch keine Options-IDs, daher muss jede an ihn gesendete Option die `id` weglassen.
- Die Umfrage‑Privatsphäre kann nach Stimmen eingeschränkt, aber nicht erweitert werden.
- Diese API beachtet Ihre Site‑Einstellungen. Wenn Umfragen für die Site oder Seite nicht aktiviert sind, schlägt sie mit `polls-disabled` fehl.
- Ein gesperrter Kommentar kann seine Umfrage nicht ändern und schlägt mit `locked` fehl.
- Verbundenen Widgets werden live aktualisiert, sodass Betrachter die neue Umfrage ohne Neuladen sehen.