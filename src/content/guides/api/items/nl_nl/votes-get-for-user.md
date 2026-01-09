[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Maakt het mogelijk om stemmen op te halen die een gebruiker heeft achtergelaten voor een bepaalde `urlId`. Vereist een `userId` dat een FastComments.com- of een `SSO User`-account kan zijn.

Dit is handig als je wilt laten zien of een gebruiker op een reactie heeft gestemd. Wanneer je reacties ophaalt, roep je simpelweg deze API tegelijkertijd aan voor de gebruiker met dezelfde `urlId`.

Als je anoniem stemmen gebruikt, geef je in plaats daarvan `anonUserId` mee.

[inline-code-attrs-start title = 'Stemmen voor gebruiker cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Stemmen voor anonieme gebruiker cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Let op dat anonieme stemmen zullen verschijnen in de `appliedAuthorizedVotes` lijst. Ze worden als geautoriseerd beschouwd omdat ze via de API met een API key zijn aangemaakt.

[inline-code-attrs-start title = 'Verzoekstructuur: Stemmen voor gebruiker'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Antwoordstructuur: Stemmen voor gebruiker'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Opgenomen bij mislukking. **/
    reason?: string
    /** Geautoriseerde, geverifieerde stemmen, toegepast op hun overeenkomstige reacties. **/
    appliedAuthorizedVotes: Vote[]
    /** Stemmen in afwachting van verificatie, nog niet toegepast op hun overeenkomstige reacties. **/
    pendingVotes: Vote[]
}
[inline-code-end]