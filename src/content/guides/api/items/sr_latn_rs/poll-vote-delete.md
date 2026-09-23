[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Povlači glas. Opcija na koju je glas bio izdat vraća svoj broj, a glasač može ponovo glasati.

[inline-code-attrs-start title = 'PollVote Delete cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Delete Struktura Zahteva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Delete Struktura Odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Anketa sa ažuriranim brojačima. **/
    poll: CommentPoll
}
[inline-code-end]

### Ostale napomene

- Brisanje istog glasa dva puta vraća `not-found` drugi put, a brojači ostaju nepromenjeni.
- Ako je anketa zamenjena od kada je glas izdat, glas se uklanja, ali se brojači ne menjaju, jer je zamena počela od nule.