[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Povlači glas. Opcija na koju je glas bio izabran vraća svoj broj, a glasač je slobodan ponovno glasati.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za brisanje PollVote'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za brisanje PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Anketa s ažuriranim brojevima. **/
    poll: CommentPoll
}
[inline-code-end]

### Ostale napomene

- Brisanje istog glasa dva puta vraća `not-found` drugi put, a brojevi ostaju nepromijenjeni.
- Ako je anketa zamijenjena od kada je glas izabran, glas se uklanja, ali se brojevi ne mijenjaju, jer je zamjena započela od nule.

---