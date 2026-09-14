[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Ova ruta stvara jedan `FeedPost`. Svaki post ima autora, pa je `fromUserId` obavezan i mora biti ID postojećeg FastComments ili SSO korisnika na računu.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahtjeva za stvaranje FeedPosta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Pošalji post na feedove koji su trenutno otvoreni u pregledniku. Zadano je false. **/
    isLive?: boolean
    /** Provedi post kroz anti-spam mehanizam prije spremanja. Zadano je false. **/
    doSpamCheck?: boolean
    /** Preskoči provjeru ponovljenog sadržaja koja se izvršava kao dio doSpamCheck. Zadano je false. **/
    skipDupCheck?: boolean
    /** Do 256 znakova. Emitira se živim slušateljima kako bi klijent mogao ignorirati vlastitu emisiju. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Obavezno. ID FastComments ili SSO korisnika. **/
    fromUserId: string
    title?: string
    /** HTML. Sanira se pri spremanju. **/
    contentHTML?: string
    /** Zamjenjuje prikazano ime uzeto od korisnika. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za stvaranje FeedPosta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    feedPost?: FeedPost; // Vraćamo cijeli kreirani post pri uspjehu.
}
[inline-code-end]