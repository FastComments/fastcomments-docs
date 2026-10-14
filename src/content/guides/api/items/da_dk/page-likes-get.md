[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Returnerer antallet af likes på en side, og om den aktuelle bruger har liket den. Sider, der endnu ikke findes, returnerer en `likeCount` på `0`.

Send `includeUserInfo=public` for også at få brugerne, der har liket siden, seneste først. Brugere med privat profilaktivitet og anonyme brugere udelades, og op til 100 likes slås op. Kun `public` accepteres her; `include-private` returnerer en `422` og er tilgængelig fra [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) med en API-nøgle. På sider, der kræver SSO, returnerer `includeUserInfo` en `403` for brugere, der ikke kan se siden.

[inline-code-attrs-start title = 'Eksempel på cURL for Page Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktur for Page Likes-anmodning'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur for Page Likes-svar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    likeCount: number
    /** Whether the user making the request has liked the page. **/
    didLike: boolean
    /** The number of top level comments on the page. **/
    commentCount: number
    /** The id used to subscribe to live updates for this page. **/
    urlIdWS: string
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]