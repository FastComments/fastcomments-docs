[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Uklanja lajk trenutnog korisnika s stranice. Ako korisnik nije lajkao stranicu, zahtjev uspijeva s kodom `not-liked` i ne mijenja broj.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za uklanjanje lajkova stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za uklanjanje lajkova stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI kodirani JSON vašeg SSO objekta. Izostavite za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje lajkova stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' kada korisnik nije lajkao stranicu. Inače je uključeno u slučaju neuspjeha. **/
    code?: 'not-liked' | string
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]