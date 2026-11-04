[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava deblokiranje korisnika koji je napisao određeni komentar. Podržava deblokiranje komentara koje su napisali FastComments.com korisnici, SSO korisnici i Tenant korisnici.

Podržava `commentIdsToCheck` body parametar za provjeru trebaju li se drugi potencijalno vidljivi komentari na klijentu blokirati/deblokirati nakon izvođenja ove radnje.

Napomene:

- Ovaj poziv uvijek se mora izvršiti u kontekstu korisnika. Korisnik može biti FastComments.com korisnik, SSO korisnik ili Tenant korisnik.
- `userId` u zahtjevu je korisnik koji *vrši deblokiranje*. Na primjer: `User A` želi deblokirati `User B`. Proslijedite `userId=User A` i ID komentara koji je `User B` napisao.
- Potpuno anonimni komentari (bez korisničkog ID-a, bez e-pošte) ne mogu se blokirati i vratit će se pogreška.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za deblokiranje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za deblokiranje anonimnog komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za deblokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora na deblokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are still blocked. If false, you might want to un-hide the comments from the user so they don't have to refresh. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]