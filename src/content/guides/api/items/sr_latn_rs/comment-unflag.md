[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava uklanjanje oznake (un-flag) sa komentara za određenog korisnika.

Napomene:

- Ovaj poziv mora uvek biti izvršen u kontekstu korisnika. Korisnik može biti FastComments.com User, SSO User, ili Tenant User.
- Nakon što je komentar automatski neodobren (sakriven) - komentar može ponovo biti odobren samo od strane administratora ili moderatora. Uklanjanje oznake (un-flag) neće ponovo odobriti komentar.

[inline-code-attrs-start title = 'cURL primer: uklanjanje oznake komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Za anonimno označavanje, moramo navesti `anonUserId`. To može biti ID koji predstavlja anonimnu sesiju, ili nasumičan UUID.

[inline-code-attrs-start title = 'cURL primer: anonimno uklanjanje oznake komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahteva za uklanjanje oznake komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje oznake komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]