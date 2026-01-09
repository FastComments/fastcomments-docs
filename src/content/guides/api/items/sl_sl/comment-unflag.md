[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Ta končna točka API-ja omogoča odstranitev zastavice (un-flag) iz komentarja za določenega uporabnika.

Opombe:

- Klic mora biti vedno izveden v kontekstu uporabnika. Uporabnik je lahko FastComments.com User, SSO User, ali Tenant User.
- Ko je komentar samodejno zavrnjen (skrit) - komentar lahko ponovno odobri le administrator ali moderator. Odznačitev zastavice ne bo ponovno odobrila komentarja.

[inline-code-attrs-start title = 'Primer cURL za odznačitev komentarja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Za anonimno označevanje z zastavico moramo navesti `anonUserId`. To je lahko ID, ki predstavlja anonimno sejo, ali naključni UUID.

[inline-code-attrs-start title = 'Primer cURL za anonimno odznačitev komentarja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahtevka za odznačitev komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za odznačitev komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Vključeno ob neuspehu. **/
    reason?: string
}
[inline-code-end]