[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Ta API endpoint omogoča blokiranje uporabnika, ki je napisal določen komentar. Podpira blokiranje komentarjev, napisanih s strani FastComments.com uporabnikov, SSO uporabnikov in najemniških uporabnikov.

Podpira telo parameter `commentIdsToCheck`, s katerim se preveri, ali je treba po tej akciji kateri koli drug potencialno viden komentar na odjemalcu blokirati/odblokirati.

Opombe:

- Ta klic mora biti vedno izveden v kontekstu uporabnika. Uporabnik je lahko FastComments.com uporabnik, SSO uporabnik ali najemniški uporabnik.
- V zahtevi `userId` označuje uporabnika, ki *izvaja blokiranje*. Na primer: `User A` želi blokirati `User B`. Posredujte `userId=User A` in ID komentarja, ki ga je napisal `User B`.
- Popolnoma anonimnih komentarjev (brez ID uporabnika, brez e-pošte) ni mogoče blokirati in bo vrnjena napaka.

[inline-code-attrs-start title = 'Primer cURL zahteve za blokiranje komentarja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Za anonimno blokiranje moramo navesti en `anonUserId`. To je lahko ID, ki predstavlja anonimno sejo, ali naključen UUID.
To nam omogoča podporo blokiranja komentarjev tudi, če uporabnik ni prijavljen, saj se komentarji pridobijo z istim `anonUserId`.

[inline-code-attrs-start title = 'Primer cURL za blokiranje anonimnega komentarja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za blokiranje komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za blokiranje komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Vključeno ob neuspehu. **/
    reason?: string
    /** Če je commentIdsToCheck definirano, so vnosi v tem zemljevidu z vrednostjo true prav tako blokirani. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]