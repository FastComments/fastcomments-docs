[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Uklanja jednu od reakcija trenutnog korisnika sa stranice. Ako korisnik nije dodao tu reakciju, zahtev uspešno prolazi sa kodom `no-react` i ne menja broj.

[inline-code-attrs-start title = 'Primer cURL zahteva za brisanje reakcije na stranici'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za brisanje reakcije na stranici'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** ID reakcije. **/
    id: string
    /** URI enkodovan JSON vašeg SSO objekta. Omitujte za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje reakcije na stranici'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' kada korisnik nije dodao ovu reakciju. U suprotnom je uključeno pri grešci. **/
    code?: 'no-react' | string
    /** Uključeno pri grešci. **/
    reason?: string
}
[inline-code-end]