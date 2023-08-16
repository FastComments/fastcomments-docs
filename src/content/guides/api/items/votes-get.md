[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Votes must be fetched by `urlId`.

### Types of Votes

There are three types of votes:

- Authenticated Votes, which are applied to the corresponding comment. You can create these via this API.
- Authenticated Votes, which are **pending** verification, and thus are not yet applied to the comment. These are created when a user uses the FastComments.com *login to vote* mechanism.
- Anonymous Votes, which are applied to the corresponding comment. These are created along with anonymous commenting.

These are returned in separate lists in the API to reduce confusion.

[inline-code-attrs-start title = 'Votes cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Votes Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Votes Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonymous votes, applied to their corresponding comments. **/
    appliedAnonymousVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Anonymous Votes Notes

Note that anonymous votes created via this API will appear in the `appliedAuthorizedVotes` list. They are considered authorized since they were created via the API with an API key.

The `appliedAnonymousVotes` structure is for votes created without an email, API key, etc.
