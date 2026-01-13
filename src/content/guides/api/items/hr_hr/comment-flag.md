[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Ova API krajnja točka pruža mogućnost označavanja komentara za određenog korisnika.

Napomene:

- Ovaj poziv se uvijek mora izvršiti u kontekstu korisnika. Korisnik može biti FastComments.com korisnik, SSO korisnik ili korisnik stanara.
- Ako je postavljen prag za skrivanje označavanjem, komentar će automatski biti skriven uživo nakon što je označen definirani broj puta.
- Nakon što je automatski skriven (neodobren) - komentar može ponovno odobriti samo administrator ili moderator. Uklanjanje oznake neće ponovno odobriti komentar.

[inline-code-attrs-start title = 'Primjer cURL za označavanje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Za anonimno označavanje, moramo navesti `anonUserId`. Ovo može biti ID koji predstavlja anonimnu sesiju ili nasumični UUID.
Ovo nam omogućuje podršku za označavanje i uklanjanje oznake s komentara čak i ako korisnik nije prijavljen. Na taj način, komentar može biti označen kao
označen kada se komentari dohvaćaju s istim `anonUserId`.

[inline-code-attrs-start title = 'Primjer cURL za anonimno označavanje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahtjeva za označavanje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
    /** Was the comment un-approved (hidden) due to being flagged too many times? **/
    wasUnapproved?: boolean;
}
[inline-code-end]
