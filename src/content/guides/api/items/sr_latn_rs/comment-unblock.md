[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava poništavanje blokiranja korisnika koji je napisao dati komentar. Podržava poništavanje blokiranja komentara koje su napisali FastComments.com korisnici, SSO korisnici i Tenant korisnici.

Podržan je body parametar `commentIdsToCheck` koji proverava da li bi ostali potencijalno vidljivi komentari na klijentu trebalo da budu blokirani/poništeni nakon što se ova akcija izvrši.

Napomene:

- Ovaj poziv mora uvek biti izvršen u kontekstu korisnika. Korisnik može biti FastComments.com korisnik, SSO korisnik ili Tenant korisnik.
- `userId` u zahtevu je korisnik koji *poništava blokiranje*. Na primer: `User A` želi da poništi blokiranje `User B`. Prosledi `userId=User A` i ID komentara koji je `User B` napisao.
- Potpuno anonimni komentari (bez user id, bez email) ne mogu biti blokirani i biće vraćena greška.

[inline-code-attrs-start title = 'Primer cURL zahteva za poništavanje blokiranja komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteva za poništavanje blokiranja anonimnog komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za poništavanje blokiranja komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za poništavanje blokiranja komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Ako je commentIdsToCheck definisan, unosi u ovoj mapi sa vrednošću true su i dalje blokirani. Ako su sa vrednošću false, možda ćete želeti da ponovo prikažete komentare korisniku kako ne bi morao da osvežava stranicu. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]