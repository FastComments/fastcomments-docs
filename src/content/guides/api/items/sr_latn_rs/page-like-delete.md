[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Uklanja lajk trenutnog korisnika sa stranice. Ako korisnik nije lajkovao stranicu, zahtev uspešno prolazi sa kodom `not-liked` i ne menja broj.

[inline-code-attrs-start title = 'Primer cURL zahteva za uklanjanje lajka'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za uklanjanje lajka'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI enkodovani JSON vašeg SSO objekta. Omitujte za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje lajka'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' kada korisnik nije lajkovao stranicu. U suprotnom je uključeno pri grešci. **/
    code?: 'not-liked' | string
    /** Uključeno pri grešci. **/
    reason?: string
}
[inline-code-end]