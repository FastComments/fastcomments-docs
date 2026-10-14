[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Vrne število všečkov na strani in ali je trenutni uporabnik všečkal stran. Strani, ki še ne obstajajo, vrnejo `likeCount` vrednost `0`.

Posredujte `includeUserInfo=public`, da dobite tudi uporabnike, ki so všečkali stran, najprej najnovejše. Uporabniki z zasebnimi aktivnostmi profila in anonimni uporabniki so izključeni, in se pregleda do 100 všečkov. Tukaj je sprejet le `public`; `include-private` vrne `422` in je na voljo prek [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) z API ključem. Na straneh, ki zahtevajo SSO, `includeUserInfo` vrne `403` za uporabnike, ki strani ne morejo ogledati.

[inline-code-attrs-start title = 'Primer cURL zahteve za všečke strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za všečke strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI kodiran JSON vašega SSO objekta. Izpustite za anonimne uporabnike. **/
    sso?: string
    /** Nastavite na 'public', da vključite uporabnike, ki so všečkali stran. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za všečke strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Vključeno ob napaki. **/
    reason?: string
    likeCount: number
    /** Ali je uporabnik, ki je poslal zahtevo, všečkal stran. **/
    didLike: boolean
    /** Število komentarjev najvišje ravni na strani. **/
    commentCount: number
    /** ID, ki se uporablja za naročanje na žive posodobitve za to stran. **/
    urlIdWS: string
    /** Najprej najnovejše. Nastavljeno le, ko je poslan includeUserInfo. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

---