[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućuje deblokiranje korisnika koji je napisao određeni komentar. Podržava deblokiranje komentara koje su napisali FastComments.com korisnici, SSO korisnici i Tenant korisnici.

Podržava body parametar `commentIdsToCheck` za provjeru trebaju li nakon ove radnje biti blokirani/deblokirani neki drugi potencijalno vidljivi komentari na klijentu.

Napomene:

- Ovaj poziv uvijek se mora izvršiti u kontekstu korisnika. Korisnik može biti FastComments.com korisnik, SSO korisnik ili Tenant korisnik.
- The `userId` in the request is the user that is *doing the un-blocking*. For example: `User A` wants to Un-Block `User B`. Pass `userId=User A` and the comment id that `User B` wrote.
- Potpuno anonimni komentari (bez ID-a korisnika, bez e-maila) ne mogu biti blokirani i bit će vraćena pogreška.

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

[inline-code-attrs-start title = 'Struktura odgovora za deblokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Uključeno kod neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Uključeno kod neuspjeha. **/
    reason?: string
    /** Ako je commentIdsToCheck definiran, unosi u ovoj mapi s vrijednošću true i dalje su blokirani. Ako su false, možda ćete htjeti ponovno prikazati komentare korisniku kako se ne bi morali osvježavati. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]