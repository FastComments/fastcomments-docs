[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Vraća imena korisnika koji su dodali reakciju na stranicu, poredana abecedno. Pretražuje se najviše 100 reakcija, a anonimni korisnici nisu uključeni.

[inline-code-attrs-start title = 'Page React Users cURL primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Page React Users struktura zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** ID reakcije. **/
    id: string
    /** URI kodirani JSON vašeg SSO objekta. Izostavite za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page React Users struktura odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: string
    /** Uključeno pri neuspjehu. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]