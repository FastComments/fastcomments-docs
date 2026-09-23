[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Lister de individuelle stemmer bag en afstemnings optælling, ældste først. Én kredit per 100 stemmer returneret.

En afstemning tilhører en kommentar, så stemmer læses én afstemning ad gangen, og `commentId` er påkrævet. Indsnævr yderligere med `voterId` for at kontrollere, hvordan én person stemte, eller med `optionId` for at liste alle, der valgte en given mulighed.

Højst 1000 stemmer returneres pr. kald. Brug `skip` for at paginere videre.

Afstemningens `privacy`-indstilling respekteres: stemmerne på en anonym afstemning kan ikke læses, og anmodningen fejler med `poll-anonymous`. Se `PollVote`-strukturen for detaljer.

[inline-code-attrs-start title = 'PollVotes Get cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVotes Get Responsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Counting Votes Per Option

Du behøver ikke at lægge dem sammen for at få resultaterne – afstemningen indeholder sine egne optællinger. Læs afstemningen med `GET /api/v1/polls/:commentId` i stedet, og brug dette API når du har brug for at vide, hvem der har stemt.

### Every Poll On A Page

Der findes ingen sideomfattende stemmeliste. For at rapportere om en hel side, hent dens kommentarer med `GET /api/v1/comments`, som returnerer hver kommentars afstemning og dens optællinger, og læs derefter stemmerne for de afstemninger, du er interesseret i.

---