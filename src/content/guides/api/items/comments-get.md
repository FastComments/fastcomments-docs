[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; costPerPageLoad = 30; api-resource-header-end]

A `Comment` object represents a comment left by a user.

The relationship between parent and child comments is defined via `parentId`;

Comments must be fetched by-page. You can fetch comments for all pages at once, but use caution.

[inline-code-attrs-start title = 'Comments cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Comments Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string;
    API_KEY: string;
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId: string;
    /** The page to fetch, starting with 0. Pass -1 for all comments. **/
    page: number;
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction: 'MR' | 'OF' | 'NF';
}
[inline-code-end]

[inline-code-attrs-start title = 'Comments Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    id: string;
    tenantId: string;
    userId?: string|null;
    urlId: string;
    commenterName: string;
    commenterLink: string;
    commentHTML: string;
    parentId?: string|null;
    date: string;
    votes: number;
    verified: boolean;
    avatarSrc: string;
    hasImages: boolean;
    hasLinks: boolean;
    isByAdmin?: boolean;
    isByModerator?: boolean;
    isPinned?: boolean;
    displayLabel?: string;
    rating?: number;
}

interface CommentsResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id';
    /** Included on failure. **/
    reason?: string;
    /** The comments! **/
    comments: Comment[];
}
[inline-code-end]

#### Helpful Tip

The `Comment` API requires a `urlId`. You can call the `Pages` API first, to see what the `urlId` values available to you
look like.
