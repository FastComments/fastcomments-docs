[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Attaches a poll to an existing comment, or sets the full state of the poll it already has.

The body is the complete poll, and the options you send become the poll's options, in that order. Each option
is matched by its `id`:

- An option sent with the `id` of an existing option keeps that option and its votes. Its label and position
  are updated to what you sent.
- An option sent without an `id` is added, with no votes.
- An existing option you leave out is removed, along with the votes cast on it. `totalVotes` drops by the same
  amount.

So to add an option, send the current options with their ids plus the new one without an id. To remove one,
send the list without it. The option ids are on the poll returned by `GET /api/v1/polls/:commentId`.

Sending no ids at all replaces every option and deletes every vote already cast on the poll. If the poll has
votes, this requires `replaceVotes=true`, and without it the API responds with `replace-votes-required`.

The other fields are replaced too: leaving out `closesAt`, `privacy` or `requireVoteToSeeResults` resets it to
its default. To change a single field and leave the rest alone, use `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Poll Put cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

- An `id` that is not on the poll, or the same `id` given twice, fails with `poll-invalid`. A comment with no
  poll has no option ids yet, so every option sent to it must leave `id` out.
- Poll privacy can be narrowed but not widened once it has votes.
- This API obeys your site settings. If polls are not enabled for the site or page, it fails with
  `polls-disabled`.
- A locked comment cannot have its poll changed, and fails with `locked`.
- Connected widgets are updated live, so viewers see the new poll without reloading.
