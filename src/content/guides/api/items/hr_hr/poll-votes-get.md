[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Izlistava pojedinačne glasove koji stoje iza brojeva jednog ankete, najstariji prvi. Jedan kredit po 100 vraćenih glasova.

Anketa pripada komentaru, pa se glasovi čitaju po jednoj anketi odjednom i `commentId` je obavezan. Dodatno suzite s `voterId` da provjerite kako je jedna osoba glasala, ili s `optionId` da izlistate sve koji su odabrali određenu opciju.

Maksimalno 1000 glasova se vraća po pozivu. Koristite `skip` za paginaciju.

Poštuje se postavka `privacy` ankete: glasovi na anonimnoj anketi ne mogu se čitati, a zahtjev ne uspijeva s `poll-anonymous`. Pogledajte strukturu `PollVote` za detalje.

[inline-code-attrs-start title = 'PollVotes Get cURL primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get struktura zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVotes Get struktura odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Ne morate ih zbrajati da biste dobili rezultate - anketa već sadrži svoje brojeve. Umjesto toga pročitajte anketu s `GET /api/v1/polls/:commentId`, i koristite ovaj API kada trebate znati tko je glasao.

### Svaka anketa na stranici

Ne postoji popis glasova za cijelu stranicu. Za izvještavanje o cijeloj stranici, dohvatite njene komentare s `GET /api/v1/comments`, što vraća anketu svakog komentara i njegove brojeve, a zatim pročitajte glasove za ankete koje vas zanimaju.