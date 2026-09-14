[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

This route creates a single `FeedPost`. Every post has an author, so `fromUserId` is required and must be the id of
an existing FastComments or SSO user on the account.

[inline-code-attrs-start title = 'FeedPost Create cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Create Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Push the post to feeds that are open in a browser right now. Defaults to false. **/
    isLive?: boolean
    /** Run the post through the spam engine before saving. Defaults to false. **/
    doSpamCheck?: boolean
    /** Skip the repeated-content check that runs as part of doSpamCheck. Defaults to false. **/
    skipDupCheck?: boolean
    /** Up to 256 characters. Echoed to live listeners so a client can ignore its own broadcast. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Required. A FastComments or SSO user id. **/
    fromUserId: string
    title?: string
    /** HTML. Sanitized on save. **/
    contentHTML?: string
    /** Overrides the display name taken from the user. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Create Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Included on failure. **/
    reason?: string
    feedPost?: FeedPost; // We return the complete created post on success.
}
[inline-code-end]
