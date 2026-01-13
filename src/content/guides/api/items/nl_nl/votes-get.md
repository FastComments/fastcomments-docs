[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Stemmen moeten worden opgehaald via `urlId`.

### Soorten stemmen

Er zijn drie soorten stemmen:

- Geauthenticeerde stemmen, die worden toegepast op de bijbehorende opmerking. Je kunt deze via deze API aanmaken.
- Geauthenticeerde stemmen, die **in afwachting** zijn van verificatie, en dus nog niet op de opmerking worden toegepast. Deze worden aangemaakt wanneer een gebruiker het FastComments.com *inloggen om te stemmen*-mechanisme gebruikt.
- Anonieme stemmen, die worden toegepast op de bijbehorende opmerking. Deze worden aangemaakt samen met anoniem reageren.

Deze worden in afzonderlijke lijsten in de API teruggegeven om verwarring te verminderen.

[inline-code-attrs-start title = 'Voorbeeld cURL-aanvraag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de Votes-aanvraag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de Votes-respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij falen. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Opgenomen bij falen. **/
    reason?: string
    /** Geautoriseerde, geverifieerde stemmen, toegepast op hun bijbehorende opmerkingen. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonieme stemmen, toegepast op hun bijbehorende opmerkingen. **/
    appliedAnonymousVotes: Vote[]
    /** Stemmen in afwachting van verificatie, nog niet toegepast op hun bijbehorende opmerkingen. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Opmerkingen over anonieme stemmen

Let op dat anonieme stemmen die via deze API zijn aangemaakt, verschijnen in de lijst `appliedAuthorizedVotes`. Ze worden als geautoriseerd beschouwd omdat ze via de API met een API key zijn aangemaakt.

De structuur `appliedAnonymousVotes` is voor stemmen die zijn aangemaakt zonder e-mail, API key, enz.

---