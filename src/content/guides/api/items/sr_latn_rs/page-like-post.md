[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Lajkuje stranicu kao trenutni korisnik. Svaki korisnik može lajkovati stranicu jednom: ponovni lajk uspešno prolazi sa kodom `already-liked` i ne menja broj.

Stranica se kreira ako još ne postoji. Prosledite `title` da postavite ili ažurirate naslov stranice.

[inline-code-attrs-start title = 'Primer cURL zahteva za lajk stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za lajk stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Postavlja naslov stranice. **/
    title?: string
    /** URI enkodovan JSON vašeg SSO objekta. Omitujte za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora na lajk stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' kada je korisnik već lajkao stranicu. U suprotnom je uključeno pri grešci. **/
    code?: 'already-liked' | string
    /** Uključeno pri grešci. **/
    reason?: string
}
[inline-code-end]