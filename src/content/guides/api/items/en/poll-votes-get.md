[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Lists the individual votes behind one poll's counts, oldest first. One credit per 100 votes returned.

A poll belongs to a comment, so votes are read one poll at a time and `commentId` is required. Narrow further
with `voterId` to check how one person voted, or with `optionId` to list everyone who picked a given option.

At most 1000 votes are returned per call. Use `skip` to page through more.

The poll's `privacy` setting is respected: the votes on an anonymous poll cannot be read, and the request fails
with `poll-anonymous`. See the `PollVote` structure for details.

[inline-code-attrs-start title = 'PollVotes Get cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Counting Votes Per Option

You do not have to add these up to get the results - the poll carries its own counts. Read the poll with
`GET /api/v1/polls/:commentId` instead, and use this API when you need to know who voted.

### Every Poll On A Page

There is no page-wide vote listing. To report on a whole page, fetch its comments with
`GET /api/v1/comments`, which returns each comment's poll and its counts, and then read the votes for the
polls you care about.
