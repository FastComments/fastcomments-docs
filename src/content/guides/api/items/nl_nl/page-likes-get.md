[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Retourneert het aantal likes op een pagina, en of de huidige gebruiker deze heeft geliket. Pagina's die nog niet bestaan, geven een `likeCount` van `0` terug.

Geef `includeUserInfo=public` mee om ook de gebruikers te krijgen die de pagina hebben geliket, meest recent eerst. Gebruikers met privéprofielactiviteit en anonieme gebruikers worden weggelaten, en er worden maximaal 100 likes opgezocht. Alleen `public` wordt hier geaccepteerd; `include-private` geeft een `422` terug en is beschikbaar via [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) met een API-sleutel. Op pagina's die SSO vereisen, geeft `includeUserInfo` een `403` terug voor gebruikers die de pagina niet kunnen bekijken.

[inline-code-attrs-start title = 'Voorbeeld cURL voor Pagina Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina Likes-verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina Likes-respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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