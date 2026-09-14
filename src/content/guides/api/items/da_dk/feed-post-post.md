[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Denne rute opretter et enkelt `FeedPost`. Hvert indlæg har en forfatter, så `fromUserId` er påkrævet og skal være id'et på en eksisterende FastComments- eller SSO-bruger på kontoen.

[inline-code-attrs-start title = 'FeedPost Opret cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'FeedPost Opret Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Skub indlægget til feeds, der er åbne i en browser lige nu. Standard er false. **/
    isLive?: boolean
    /** Kør indlægget gennem spam-motoren før gemning. Standard er false. **/
    doSpamCheck?: boolean
    /** Spring gentaget-indhold tjekket over, som kører som en del af doSpamCheck. Standard er false. **/
    skipDupCheck?: boolean
    /** Op til 256 tegn. Ekkoet til live-lyttere så en klient kan ignorere sin egen udsendelse. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Påkrævet. Et FastComments- eller SSO-bruger-id. **/
    fromUserId: string
    title?: string
    /** HTML. Renset ved gemning. **/
    contentHTML?: string
    /** Overskriver display-navnet hentet fra brugeren. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Opret Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Inkluderet ved fejl. **/
    reason?: string
    feedPost?: FeedPost; // Vi returnerer det komplette oprettede indlæg ved succes.
}
[inline-code-end]