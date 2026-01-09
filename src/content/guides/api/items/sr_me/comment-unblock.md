[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava uklanjanje blokade korisniku koji je napisao određeni komentar. Podržava uklanjanje blokade za komentare koje su napisali FastComments.com korisnici, SSO korisnici, i Tenant korisnici.

Podržava body parametar `commentIdsToCheck` koji provjerava da li bi ostali potencijalno vidljivi komentari na klijentu trebali biti blokirani/otblokirani nakon izvršene akcije.

Napomene:

- Ovaj poziv mora uvijek biti napravljen u kontekstu korisnika. Korisnik može biti FastComments.com korisnik, SSO korisnik, ili Tenant korisnik.
- Polje `userId` u zahtjevu je korisnik koji je *izvodi otblokiranje*. Na primjer: `User A` želi otblokirati `User B`. Proslijedite `userId=User A` i id komentara koji je napisao `User B`.
- Potpuno anonimni komentari (bez user id, bez email) ne mogu biti blokirani i biće vraćena greška.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za otblokiranje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za otblokiranje anonimnog komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za otblokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za otblokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Ako je commentIdsToCheck definisan, unosi u ovoj mapi sa vrijednošću true su i dalje blokirani. Ako su false, možda ćete htjeti ponovo prikazati komentare korisniku kako ne bi morali osvježavati. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]