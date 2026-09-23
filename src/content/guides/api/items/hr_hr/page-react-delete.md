[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Uklanja jednu od reakcija trenutnog korisnika s stranice. Ako korisnik nije dodao tu reakciju, zahtjev uspijeva s kodom `no-react` i ne mijenja broj.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za brisanje reakcije na stranici'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za brisanje reakcije na stranici'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje reakcije na stranici'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' when the user had not added this reaction. Otherwise included on failure. **/
    code?: 'no-react' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]