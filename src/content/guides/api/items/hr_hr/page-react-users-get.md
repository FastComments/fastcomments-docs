[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Vraća imena korisnika koji su dodali reakciju na stranicu, sortirana abecedno. Pretražuje se najviše 100 reakcija, a anonimni korisnici i korisnici s privatnom aktivnošću profila nisu uključeni. Na stranicama koje zahtijevaju SSO, korisnici koji ne mogu pregledati stranicu dobivaju `403` s kodom `sso-required` ili `page-forbidden`.

Za dobivanje avatara i ID‑ova korisnika, koristite [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) s `includeUserInfo=public`.

[inline-code-attrs-start title = 'Primjer cURL za korisnike reakcija na stranicu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za korisnike reakcija na stranicu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za korisnike reakcija na stranicu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]