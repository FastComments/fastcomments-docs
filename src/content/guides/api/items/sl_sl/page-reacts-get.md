[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Vrne število za vsako reakcijo na strani in katere reakcije je trenutni uporabnik dodal.

[inline-code-attrs-start title = 'Primer cURL za Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI kodiran JSON vašega SSO objekta. Izpustite za anonimne uporabnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: string
    /** Vključeno ob napaki. **/
    reason?: string
    /** Število po ID-ju reakcije, na primer {"heart": 12, "laugh": 3}. Ni nastavljeno, ko stran nima reakcij. **/
    counts?: Record<string, number>
    /** ID-ji reakcij, ki jih je uporabnik, ki je poslal zahtevo, dodal. Ni nastavljeno, če jih ni dodal. **/
    reactedIds?: string[]
}
[inline-code-end]