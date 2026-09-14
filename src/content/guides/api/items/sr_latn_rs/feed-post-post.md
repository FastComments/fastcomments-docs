[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Ova ruta kreira jedan `FeedPost`. Svaki post ima autora, pa je `fromUserId` obavezan i mora biti ID postojećeg FastComments ili SSO korisnika na nalogu.

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje FeedPost-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteva za kreiranje FeedPost-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Pošaljite post u feedove koji su trenutno otvoreni u pretraživaču. Podrazumevano je false. **/
    isLive?: boolean
    /** Prođite post kroz spam motor pre čuvanja. Podrazumevano je false. **/
    doSpamCheck?: boolean
    /** Preskočite proveru ponovljenog sadržaja koja se izvršava kao deo doSpamCheck. Podrazumevano je false. **/
    skipDupCheck?: boolean
    /** Do 256 karaktera. Emituje se živim slušaocima kako klijent može da ignoriše sopstveno emitovanje. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Obavezno. ID FastComments ili SSO korisnika. **/
    fromUserId: string
    title?: string
    /** HTML. Saniran prilikom čuvanja. **/
    contentHTML?: string
    /** Zamenjuje prikazano ime preuzeto od korisnika. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za kreiranje FeedPost-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Uključeno u slučaju greške. **/
    reason?: string
    feedPost?: FeedPost; // Vraćamo kompletan kreirani post pri uspehu.
}
[inline-code-end]

---