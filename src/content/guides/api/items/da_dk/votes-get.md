[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Stemmer skal hentes via `urlId`.

### Typer af Stemmer

Der er tre typer stemmer:

- Autentificerede Stemmer, som anvendes på den tilsvarende kommentar. Du kan oprette disse via dette API.
- Autentificerede Stemmer, som **afventer** verifikation, og derfor endnu ikke er anvendt på kommentaren. Disse oprettes, når en bruger bruger FastComments.com's *log ind for at stemme*-mekanisme.
- Anonyme Stemmer, som anvendes på den tilsvarende kommentar. Disse oprettes sammen med anonym kommentering.

Disse returneres i separate lister i API'et for at reducere forvirring.

[inline-code-attrs-start title = 'Votes cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Votes Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Votes Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Bemærkninger om Anonyme Stemmer

Bemærk at anonyme stemmer oprettet via dette API vil fremgå i `appliedAuthorizedVotes`-listen. De betragtes som autoriserede, da de blev oprettet via API'et med en API-nøgle.

`appliedAnonymousVotes`-strukturen er til stemmer oprettet uden en e-mail, API-nøgle osv.
