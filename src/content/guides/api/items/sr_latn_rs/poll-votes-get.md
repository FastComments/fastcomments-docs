[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Izlistava pojedinačne glasove iza broja glasova jedne ankete, najstarije prvo. Jedan kredit po 100 vraćenih glasova.

Anketa pripada komentaru, pa se glasovi čitaju po jednoj anketi odjednom i `commentId` je obavezan. Dodatno suzite pretragu sa `voterId` da proverite kako je jedna osoba glasala, ili sa `optionId` da izlistate sve koji su odabrali određenu opciju.

Maksimalno 1000 glasova se vraća po pozivu. Koristite `skip` da paginirate dalje.

Poštuje se podešavanje `privacy` ankete: glasovi na anonimnoj anketi ne mogu biti pročitani, i zahtev ne uspeva sa `poll-anonymous`. Pogledajte strukturu `PollVote` za detalje.

[inline-code-attrs-start title = 'PollVotes Get cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Struktura Zahteva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVotes Get Struktura Odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Brojanje glasova po opciji

Ne morate da sabirate ovo da biste dobili rezultate - anketa nosi svoje sopstvene brojeve. Umesto toga pročitajte anketu sa `GET /api/v1/polls/:commentId`, i koristite ovaj API kada treba da znate ko je glasao.

### Svaka anketa na stranici

Ne postoji listanje glasova za celu stranicu. Da biste izveštavali o celoj stranici, preuzmite njene komentare sa `GET /api/v1/comments`, što vraća anketu svakog komentara i njegove brojeve, a zatim pročitajte glasove za ankete koje vas zanimaju.

---