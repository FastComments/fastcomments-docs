[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Vrne število za vsako reakcijo na strani in katere reakcije je trenutni uporabnik dodal.

Posredujte `includeUserInfo=public`, da dobite tudi uporabnike, ki so reagirali, kot en vnos na reakcijo z njenim ID-jem, najprej najnovejše. Uporabniki z zasebnimi aktivnostmi profila in anonimni uporabniki so izpuščeni, in do 100 reakcij se pregleda. Tukaj je sprejet le `public`; `include-private` vrne `422` in je na voljo iz [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) z API ključem. Na straneh, ki zahtevajo SSO, `includeUserInfo` vrne `403` za uporabnike, ki ne morejo videti strani.

[inline-code-attrs-start title = 'Primer cURL zahteve za Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Nastavite na 'public', da vključite uporabnike, ki so reagirali. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Vključeno ob napaki. **/
    reason?: string
    /** Število po ID-ju reakcije, na primer {"heart": 12, "laugh": 3}. Ni nastavljeno, ko stran nima reakcij. **/
    counts?: Record<string, number>
    /** ID-ji reakcij, ki jih je uporabnik, ki je poslal zahtevo, dodal. Ni nastavljeno, če jih ni dodal. **/
    reactedIds?: string[]
    /** En vnos na reakcijo, najprej najnovejše. Nastavljeno le, ko je poslan parameter includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** ID reakcije, na primer "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

---