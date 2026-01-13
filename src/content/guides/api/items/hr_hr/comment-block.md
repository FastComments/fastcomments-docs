[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Ova API krajnja točka pruža mogućnost blokiranja korisnika koji je napisao dani komentar. Podržava blokiranje iz komentara koje su napisali FastComments.com korisnici, SSO korisnici i korisnici stanara.

Podržava parametar tijela `commentIdsToCheck` za provjeru treba li bilo koji drugi potencijalno vidljivi komentari na klijentu biti blokirani/odblokirani nakon izvršenja ove radnje.

Napomene:

- Ovaj poziv se uvijek mora izvršiti u kontekstu korisnika. Korisnik može biti FastComments.com korisnik, SSO korisnik ili korisnik stanara.
- `userId` u zahtjevu je korisnik koji *vrši blokiranje*. Na primjer: `Korisnik A` želi blokirati `Korisnika B`. Proslijedite `userId=Korisnik A` i ID komentara koji je napisao `Korisnik B`.
- Potpuno anonimni komentari (bez ID-a korisnika, bez e-pošte) ne mogu se blokirati i bit će vraćena pogreška.

[inline-code-attrs-start title = 'Primjer cURL za blokiranje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Za anonimno blokiranje, moramo navesti `anonUserId`. Ovo može biti ID koji predstavlja anonimnu sesiju ili nasumični UUID.
Ovo nam omogućuje podršku za blokiranje komentara čak i ako korisnik nije prijavljen dohvaćanjem komentara s istim `anonUserId`.

[inline-code-attrs-start title = 'Primjer cURL za anonimno blokiranje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za blokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are also blocked. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
