[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Vraća imena korisnika koji su dodali reakciju na stranicu, sortirano abecedno. Pretražuje se do 100 reakcija, a anonimni korisnici nisu uključeni. Na stranicama koje zahtevaju SSO, korisnici koji ne mogu da vide stranicu dobijaju `403` sa kodom `sso-required` ili `page-forbidden`.

Da biste takođe dobili avatare i ID‑ove korisnika, koristite [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) sa `includeUserInfo=public`.

[inline-code-attrs-start title = 'Primer cURL zahteva za korisnike reakcija na stranicu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za korisnike reakcija na stranicu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** ID reakcije. **/
    id: string
    /** URI‑kodirani JSON vašeg SSO objekta. Izostavite za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za korisnike reakcija na stranicu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Uključeno pri grešci. **/
    code?: string
    /** Uključeno pri grešci. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]