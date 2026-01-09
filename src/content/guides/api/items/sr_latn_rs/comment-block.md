[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava blokiranje korisnika koji je napisao određeni komentar. Podržava blokiranje komentara koje su napisali FastComments.com Users, SSO Users i Tenant Users.

Podržava `commentIdsToCheck` body parametar za proveru da li bi bilo koji drugi potencijalno vidljivi komentari na klijentu trebali biti blokirani/odblokirani nakon izvođenja ove akcije.

Beleške:

- Ovaj poziv mora uvek biti izvršen u kontekstu korisnika. Korisnik može biti FastComments.com User, SSO User ili Tenant User.
- `userId` u zahtevu je korisnik koji *izvršava blokiranje*. Na primer: `User A` želi da blokira `User B`. Prosledi `userId=User A` i id komentara koji je `User B` napisao.
- Potpuno anonimni komentari (bez user id, bez email-a) ne mogu biti blokirani i vratiće se greška.

[inline-code-attrs-start title = 'Primer cURL zahteva za blokiranje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Za anonimno blokiranje, moramo navesti `anonUserId`. To može biti ID koji predstavlja anonimnu sesiju, ili nasumični UUID.
Ovo nam omogućava podršku za blokiranje komentara čak i ako korisnik nije prijavljen, tako što ćemo dovući komentare sa istim `anonUserId`.

[inline-code-attrs-start title = 'Primer cURL zahteva za blokiranje anonimnog komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za blokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za blokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Ako je commentIdsToCheck definisan, unosi u ovoj mapi sa vrednošću true su takođe blokirani. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]