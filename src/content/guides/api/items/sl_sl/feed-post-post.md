[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Ta končna točka ustvari en sam `FeedPost`. Vsaka objava ima avtorja, zato je `fromUserId` obvezen in mora biti ID obstoječega FastComments ali SSO uporabnika v računu.

[inline-code-attrs-start title = 'Primer cURL za ustvarjanje FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteve za ustvarjanje FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Pošlji objavo v vire, ki so trenutno odprti v brskalniku. Privzeto je false. **/
    isLive?: boolean
    /** Izvedi objavo skozi sistem za zaznavanje spama pred shranjevanjem. Privzeto je false. **/
    doSpamCheck?: boolean
    /** Preskoči preverjanje podvojenega vsebine, ki se izvaja kot del doSpamCheck. Privzeto je false. **/
    skipDupCheck?: boolean
    /** Do 256 znakov. Poslano poslušalcem v živo, da lahko odjemalec ignorira svoj lasten prenos. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Obvezno. ID FastComments ali SSO uporabnika. **/
    fromUserId: string
    title?: string
    /** HTML. Očiščen ob shranjevanju. **/
    contentHTML?: string
    /** Prepisuje prikazno ime, ki je vzeto od uporabnika. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora pri ustvarjanju FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Vključeno ob napaki. **/
    reason?: string
    feedPost?: FeedPost; // Vrnjemo celotno ustvarjeno objavo ob uspehu.
}
[inline-code-end]

---