[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Kommentarer skal hentes efter `urlId`. Paginering leveres af `skip`-parameteren. Kommentarer returneres i sider af `100`, sorteret efter `createdAt` nyeste først.

Prisen er baseret på antallet af returnerede kommentarer, koste `1 kredit pr. 100` returnerede kommentarer.

[inline-code-attrs-start title = 'Comment cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Comments Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId?: string
    userId?: string
    /** An optional external id to filter by. **/
    externalId?: string
    /** An optional anonymous user id to filter by. **/
    anonUserId?: string
    /** The number of comments to skip for pagination. **/
    skip?: number
    /** Sort dir (asc or desc). Default is desc. **/
    sortDirection?: 'asc' | 'desc'
    /** The start date to filter by. Comments on or after this date will be returned. **/
    startDate?: Date
    /** The end date to filter by. Comments before this date will be returned. **/
    endDate?: Date
    /** You can set this to get comments for all pages, across all url ids. This will make the urlId parameter optional. **/
    allPages?: boolean
    /** You can set this to include deleted or spam comments. **/
    includeDeleted?: boolean
    /** You can set this to only include deleted or spam comments. **/
    onlyDeleted?: boolean
    /** Include this to include the raw markdown in the response as "commentText". **/
    includeRawCommentText?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Comments Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    comments: Comment[]
}
[inline-code-end]

#### Bemærkning om Dato Filtrering

Når du filtrerer efter dato, venligst URL-enkod JSON datoformatet. For eksempel `2021-01-01T00:00:00.000Z` bliver til `2021-01-01T00%3A00%3A00.000Z`.

#### Nyttig Fif

Det anbefales stærkt at sætte `allPages` til `true` og paginere gennem alle sider for at få alle kommentarer for en given tenant.
Især hvis du automatisk opretter sider til SEO-formål.
