[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Ta API konecna točka omogoča odblokiranje uporabnika, ki je napisal določen komentar. Podpira odblokiranje komentarjev, napisanih s strani uporabnikov FastComments.com, SSO uporabnikov in najemniških uporabnikov.

Podpira parametru v telesu zahteve `commentIdsToCheck`, s katerim se preveri, ali je treba tudi druge morebitno vidne komentarje na odjemalcu po tej akciji blokirati/odblokirati.

Opombe:

- Ta klic mora biti vedno izveden v kontekstu uporabnika. Uporabnik je lahko uporabnik FastComments.com, SSO uporabnik ali najemniški uporabnik.
- V zahtevku `userId` označuje uporabnika, ki izvaja *odblokiranje*. Na primer: `User A` želi odblokirati `User B`. Posredujte `userId=User A` in ID komentarja, ki ga je napisal `User B`.
- Popolnoma anonimnih komentarjev (brez ID uporabnika, brez e-pošte) ni mogoče blokirati in bo vrnjena napaka.

[inline-code-attrs-start title = 'Primer cURL zahteve za odblokiranje komentarja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteve za odblokiranje anonimnega komentarja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za odblokiranje komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za odblokiranje komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Če je `commentIdsToCheck` določen, so vnosi v tem zemljevidu z vrednostjo true še vedno blokirani. Če false, boste morda želeli komentare ponovno prikazati uporabniku, da se mu ni treba osveževati. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]