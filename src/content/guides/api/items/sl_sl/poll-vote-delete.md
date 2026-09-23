[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Umakne glas. Možnost, na katero je bil glas oddan, dobi nazaj svoje število, in volilec je spet svoboden glasovati.

[inline-code-attrs-start title = 'PollVote Izbriši cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Izbriši strukturo zahteve'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Izbriši strukturo odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Anketa s posodobljenimi števci. **/
    poll: CommentPoll
}
[inline-code-end]

### Ostale opombe

- Brisanje istega glasu dvakrat vrne `not-found` drugič, števci pa ostanejo nespremenjeni.
- Če je bila anketa zamenjana od trenutka, ko je bil glas oddan, se glas odstrani, vendar se števci ne spremenijo, saj je zamenjava začela z ničlo.