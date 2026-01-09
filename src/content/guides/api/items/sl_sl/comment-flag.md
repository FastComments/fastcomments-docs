[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Ta API endpoint omogoča prijavo (flag) komentarja za določenega uporabnika.

Opombe:

- Ta klic mora biti vedno izveden v kontekstu uporabnika. Uporabnik je lahko FastComments.com uporabnik, SSO uporabnik ali uporabnik najemnika.
- Če je nastavljen flag-to-hide threshold, bo komentar samodejno skrit v živo, potem ko je bil prijavljen tolikokrat, kot je določeno.
- Ko je samodejno preklican (skrit) — komentar lahko ponovno potrdi le skrbnik ali moderator. Odstranitev oznake (un-flagging) komentarja ga ne bo ponovno potrdila.

[inline-code-attrs-start title = 'Primer cURL zahteve za označitev komentarja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Za anonimno prijavo moramo določiti `anonUserId`. To je lahko ID, ki predstavlja anonimno sejo, ali naključni UUID.
To nam omogoča podporo prijavljanja in odstranjevanja prijav komentarjev tudi, če uporabnik ni prijavljen. Tako je lahko komentar označen kot
prijavljen, ko se komentarji pridobijo z istim `anonUserId`.

[inline-code-attrs-start title = 'Primer cURL zahteve za anonimno označitev komentarja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahteve za označitev komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za označitev komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Ali je bil komentar samodejno preklican (skrit) zaradi prevelikega števila prijav? **/
    wasUnapproved?: boolean;
}
[inline-code-end]

---