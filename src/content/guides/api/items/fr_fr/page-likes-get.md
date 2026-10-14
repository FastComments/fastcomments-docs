[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Renvoie le nombre de likes sur une page, ainsi que le fait que l'utilisateur actuel l'ait aimée. Les pages qui n'existent pas encore renvoient un `likeCount` de `0`.

Passez `includeUserInfo=public` pour obtenir également les utilisateurs qui ont aimé la page, les plus récents en premier. Les utilisateurs avec une activité de profil privé et les utilisateurs anonymes sont exclus, et jusqu'à 100 likes sont recherchés. Seul `public` est accepté ici ; `include-private` renvoie un `422` et est disponible via [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) avec une clé API. Sur les pages qui nécessitent le SSO, `includeUserInfo` renvoie un `403` pour les utilisateurs qui ne peuvent pas voir la page.

[inline-code-attrs-start title = 'Exemple cURL des likes de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête des likes de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse des likes de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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