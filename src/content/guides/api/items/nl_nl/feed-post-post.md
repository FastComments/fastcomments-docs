[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Deze route maakt een enkele `FeedPost` aan. Elke post heeft een auteur, dus `fromUserId` is vereist en moet de id zijn van een bestaande FastComments- of SSO-gebruiker op het account.

[inline-code-attrs-start title = 'FeedPost Aanmaken cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'FeedPost Aanmaak Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Zet de post naar feeds die nu open zijn in een browser. Standaard false. **/
    isLive?: boolean
    /** Verwerk de post door de spamengine voordat deze wordt opgeslagen. Standaard false. **/
    doSpamCheck?: boolean
    /** Sla de controle op herhaalde inhoud over die wordt uitgevoerd als onderdeel van doSpamCheck. Standaard false. **/
    skipDupCheck?: boolean
    /** Tot 256 tekens. Teruggezonden naar live luisteraars zodat een client zijn eigen uitzending kan negeren. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Vereist. Een FastComments- of SSO-gebruikers-id. **/
    fromUserId: string
    title?: string
    /** HTML. Gesanitiseerd bij opslaan. **/
    contentHTML?: string
    /** Overschrijft de weergavenaam die van de gebruiker is genomen. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Aanmaak Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Inbegrepen bij falen. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Inbegrepen bij falen. **/
    reason?: string
    feedPost?: FeedPost; // We retourneren de volledige aangemaakte post bij succes.
}
[inline-code-end]