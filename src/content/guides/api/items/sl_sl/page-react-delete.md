[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Odstrani eno od reakcij trenutnega uporabnika s strani. Če uporabnik ni dodal te reakcije, zahteva uspešno zaključi s kodo `no-react` in ne spremeni števila.

[inline-code-attrs-start title = 'Primer cURL zahteve za brisanje reakcije strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za brisanje reakcije strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** ID reakcije. **/
    id: string
    /** URI kodiran JSON vašega SSO objekta. Izpustite za anonimne uporabnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje reakcije strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react', ko uporabnik ni dodal te reakcije. V nasprotnem primeru je vključeno pri napaki. **/
    code?: 'no-react' | string
    /** Vključeno pri napaki. **/
    reason?: string
}
[inline-code-end]

---