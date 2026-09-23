[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Lajka stranicu kao trenutni korisnik. Svaki korisnik može lajkati stranicu jednom: ponovni lajk uspijeva s kodom `already-liked` i ne mijenja broj.

Stranica se kreira ako još ne postoji. Proslijedite `title` za postavljanje ili ažuriranje naslova stranice.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za lajk stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za lajk stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Postavlja naslov stranice. **/
    title?: string
    /** URI kodirani JSON vašeg SSO objekta. Izostavite za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za lajk stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' kada je korisnik već lajkao stranicu. Inače uključeno pri neuspjehu. **/
    code?: 'already-liked' | string
    /** Uključeno pri neuspjehu. **/
    reason?: string
}
[inline-code-end]