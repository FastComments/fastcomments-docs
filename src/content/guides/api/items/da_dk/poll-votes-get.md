[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Lister de enkelte stemmer bag én afstemnings tællinger, ældste først. Én kredit pr. 100 returnerede stemmer.

En afstemning hører til en kommentar, så stemmer læses én afstemning ad gangen, og `commentId` er påkrævet. Indsnævr yderligere
med `voterId` for at se, hvordan én person stemte, eller med `optionId` for at liste alle, der valgte en given mulighed.

Højst 1000 stemmer returneres pr. kald. Brug `skip` til at bladre gennem flere.

Afstemningens `privacy`-indstilling respekteres: stemmerne på en anonym afstemning kan ikke læses, og anmodningen fejler
med `poll-anonymous`. Se `PollVote`-strukturen for detaljer.

[inline-code-attrs-start title = 'PollVotes Get cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Forespørgselsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVotes Get Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Tælling af stemmer pr. mulighed

Du behøver ikke lægge disse sammen for at få resultaterne - afstemningen indeholder sine egne tællinger. Læs afstemningen med
`GET /api/v1/polls/:commentId` i stedet, og brug denne API, når du har brug for at vide, hvem der stemte.

### Alle afstemninger på en side

Der findes ingen sideomfattende stemmeliste. For at rapportere om en hel side, hent dens kommentarer med
`GET /api/v1/comments`, som returnerer hver kommentars afstemning og dens tællinger, og læs derefter stemmerne for de
afstemninger, du er interesseret i.