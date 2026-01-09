[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava označavanje (flag) komentara za određenog korisnika.

Napomene:

- Ovaj poziv mora uvek biti izvršen u kontekstu korisnika. Korisnik može biti FastComments.com korisnik, SSO korisnik, ili Tenant korisnik.
- Ako je postavljen prag za sakrivanje prilikom dostizanja broja oznaka, komentar će biti automatski sakriven uživo nakon što je označen definisani broj puta.
- Nakon što je automatski neodobren (sakriven) — komentar može ponovo odobriti samo administrator ili moderator. Uklanjanje oznake neće ponovo odobriti komentar.

[inline-code-attrs-start title = 'Primer cURL zahteva za označavanje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Za anonimno označavanje, moramo navesti `anonUserId`. Ovo može biti ID koji predstavlja anonimnu sesiju, ili nasumični UUID.
Ovo nam omogućava podršku za označavanje i uklanjanje oznake komentara čak i ako korisnik nije prijavljen. Na taj način, komentar može biti prikazan kao označen kada se komentari dohvate sa istim `anonUserId`.

[inline-code-attrs-start title = 'Primer cURL zahteva za anonimno označavanje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahteva za označavanje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za označavanje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Uključeno pri neuspehu. **/
    reason?: string
    /** Da li je komentar bio neodobren (sakriven) zbog toga što je bio označen previše puta? **/
    wasUnapproved?: boolean;
}
[inline-code-end]