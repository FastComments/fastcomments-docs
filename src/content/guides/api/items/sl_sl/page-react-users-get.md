[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Vrne imena uporabnikov, ki so na strani dodali reakcijo, urejena po abecedi. Pregleda se do 100 reakcij, anonimni uporabniki niso vključeni. Na straneh, ki zahtevajo SSO, uporabniki, ki strani ne morejo ogledati, prejmejo `403` s kodo `sso-required` ali `page-forbidden`.

Za pridobitev avatarjev in ID-jev uporabnikov uporabite [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) z `includeUserInfo=public`.

[inline-code-attrs-start title = 'Page React Users cURL primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** ID reakcije. **/
    id: string
    /** URI kodiran JSON vašega SSO objekta. Izpustite za anonimne uporabnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: string
    /** Vključeno ob napaki. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]