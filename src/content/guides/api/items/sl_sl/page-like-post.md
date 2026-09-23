[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Všečkanje strani kot trenutni uporabnik. Vsak uporabnik lahko stran všečka le enkrat: ponovni poskus všečkanja uspe z oznako `already-liked` in ne spremeni števila.

Stran se ustvari, če še ne obstaja. Posredujte `title`, da nastavite ali posodobite naslov strani.

[inline-code-attrs-start title = 'Primer cURL zahteve za všečkanje strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za všečkanje strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Nastavi naslov strani. **/
    title?: string
    /** URI kodiran JSON vašega SSO objekta. Izpustite za anonimne uporabnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za všečkanje strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' ko je uporabnik že všečkal stran. V nasprotnem primeru je vključeno pri napaki. **/
    code?: 'already-liked' | string
    /** Vključeno pri napaki. **/
    reason?: string
}
[inline-code-end]