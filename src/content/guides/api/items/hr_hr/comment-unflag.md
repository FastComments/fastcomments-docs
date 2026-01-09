[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint pruža mogućnost uklanjanja označavanja (un-flag) komentara za određenog korisnika.

Napomene:

- Ovaj poziv uvijek se mora izvršiti u kontekstu korisnika. Korisnik može biti FastComments.com User, SSO User, ili Tenant User.
- Nakon što je komentar automatski odbijen (sakriven) - komentar može ponovno odobriti samo administrator ili moderator. Uklanjanje oznake (un-flagging) neće ponovno odobriti komentar.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za uklanjanje oznake komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Za anonimno označavanje, moramo navesti `anonUserId`. To može biti ID koji predstavlja anonimnu sesiju, ili nasumični UUID.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za anonimno uklanjanje oznake komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahtjeva za uklanjanje oznake komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]