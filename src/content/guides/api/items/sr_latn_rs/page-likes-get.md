[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Vraća broj lajkova na stranici i da li je trenutni korisnik lajkao stranicu. Stranice koje još ne postoje vraćaju `likeCount` od `0`.

[inline-code-attrs-start title = 'Primer cURL zahteva za Page Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI enkodovani JSON vašeg SSO objekta. Omitujte za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: string
    /** Uključeno u slučaju greške. **/
    reason?: string
    likeCount: number
    /** Da li je korisnik koji je poslao zahtev lajkao stranicu. **/
    didLike: boolean
    /** Broj komentara najvišeg nivoa na stranici. **/
    commentCount: number
    /** ID koji se koristi za pretplatu na live ažuriranja za ovu stranicu. **/
    urlIdWS: string
}
[inline-code-end]

---