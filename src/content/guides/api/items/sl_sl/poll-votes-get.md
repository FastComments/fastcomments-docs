[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Izpiše posamezne glasove za štetje ankete, najstarejše najprej. En kredit na 100 vrnjenih glasov.

Anketa pripada komentarju, zato se glasovi berejo po eni anketi naenkrat in je zahtevan `commentId`. Dodatno zožite z `voterId`, da preverite, kako je posamezna oseba glasovala, ali z `optionId`, da izpišete vse, ki so izbrali določeno možnost.

Naenkrat se vrne največ 1000 glasov. Uporabite `skip` za straničenje naprej.

Nastavitev `privacy` ankete se spoštuje: glasovi na anonimni anketi ne morejo biti prebrani, zahteva pa odpove z `poll-anonymous`. Za podrobnosti si oglejte strukturo `PollVote`.

[inline-code-attrs-start title = 'PollVotes Get cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Struktura Zahteve'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVotes Get Struktura Odziva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Vključeno ob napaki. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Štetje glasov po možnosti

Ni potrebno seštevati teh glasov za rezultate – anketa že vsebuje svoje štetje. Namesto tega preberite anketo z `GET /api/v1/polls/:commentId` in uporabite ta API, ko morate vedeti, kdo je glasoval.

### Vsaka anketa na strani

Na strani ni celotnega seznama glasov. Za poročilo o celotni strani pridobite njene komentarje z `GET /api/v1/comments`, ki vrne anketo vsakega komentarja in njeno štetje, nato pa preberite glasove za ankete, ki vas zanimajo.