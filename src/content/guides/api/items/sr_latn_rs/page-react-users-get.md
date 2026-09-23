[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Vraća imena korisnika koji su dodali reakciju na stranicu, sortirano abecedno. Pretražuje se do 100 reakcija, a anonimni korisnici nisu uključeni.

[inline-code-attrs-start title = 'Primer cURL zahteva za Page React Users'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** ID reakcije. **/
    id: string
    /** URI enkodovan JSON vašeg SSO objekta. Omitujte za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: string
    /** Uključeno u slučaju greške. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]

---