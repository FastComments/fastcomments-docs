[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Lijst de individuele stemmen achter de tellingen van één poll, oudste eerst. Eén credit per 100 teruggegeven stemmen.

Een poll behoort tot een commentaar, dus stemmen worden per poll gelezen en `commentId` is vereist. Verfijn verder met `voterId` om te controleren hoe één persoon heeft gestemd, of met `optionId` om iedereen te tonen die een bepaalde optie heeft gekozen.

Maximaal 1000 stemmen worden per oproep teruggegeven. Gebruik `skip` om door meer pagina's te bladeren.

De poll's `privacy`-instelling wordt gerespecteerd: de stemmen op een anonieme poll kunnen niet worden gelezen, en het verzoek mislukt met `poll-anonymous`. Zie de `PollVote`-structuur voor details.

[inline-code-attrs-start title = 'PollVotes Get cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Request-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Response-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Inbegrepen bij falen. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Inbegrepen bij falen. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Stemmen per optie tellen

U hoeft deze niet op te tellen om de resultaten te krijgen - de poll bevat zijn eigen tellingen. Lees de poll in plaats daarvan met `GET /api/v1/polls/:commentId`, en gebruik deze API wanneer u wilt weten wie heeft gestemd.

### Elke poll op een pagina

Er is geen overzicht van stemmen voor de hele pagina. Om een volledige pagina te rapporteren, haal de commentaren op met `GET /api/v1/comments`, die voor elk commentaar de poll en de tellingen retourneert, en lees vervolgens de stemmen voor de polls die u interesseren.