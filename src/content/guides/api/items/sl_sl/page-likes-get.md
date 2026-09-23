[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Vrne število všečkov na strani in ali je trenutni uporabnik všečkal stran. Strani, ki še ne obstajajo, vrnejo `likeCount` vrednost `0`.

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
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za všečke strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Vključeno v primeru napake. **/
    code?: string
    /** Vključeno v primeru napake. **/
    reason?: string
    likeCount: number
    /** Ali je uporabnik, ki je poslal zahtevo, všečkal stran. **/
    didLike: boolean
    /** Število komentarjev najvišje ravni na strani. **/
    commentCount: number
    /** ID, ki se uporablja za naročanje na žive posodobitve za to stran. **/
    urlIdWS: string
}
[inline-code-end]

---