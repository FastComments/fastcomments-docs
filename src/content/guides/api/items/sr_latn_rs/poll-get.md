[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Čita anketu priloženu komentaru, sa trenutnim brojem glasova.

Ankete se takođe vraćaju uz sam komentar putem API-ja za komentare, pa koristite ovo kada želite samo rezultate, a ne ceo komentar.

[inline-code-attrs-start title = 'Primer cURL zahteva za Poll Get'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Poll Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Poll Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Uključeno u slučaju greške. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Komentar koji nema anketu, komentar koji je obrisan i ID komentara koji ne postoji svi odgovaraju na isti način, sa `poll-not-found`.

---