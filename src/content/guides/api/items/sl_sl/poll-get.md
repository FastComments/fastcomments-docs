[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Prebere anketo, priloženo komentarju, z njenimi trenutnimi številkami glasov.

Ankete so prav tako vrnjene v samem komentarju prek API-jev za komentarje, zato uporabite to, ko želite le rezultate in ne celotnega komentarja.

[inline-code-attrs-start title = 'Primer cURL zahteve za anketo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za pridobitev ankete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za pridobitev ankete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Vključeno v primeru napake. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Vključeno v primeru napake. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Komentar, ki nima ankete, komentar, ki je bil izbrisan, in ID komentarja, ki ne obstaja, vsi odgovorijo na enak način, z `poll-not-found`.