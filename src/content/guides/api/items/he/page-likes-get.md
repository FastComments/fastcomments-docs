[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

מחזיר את מספר הלייקים בעמוד, והאם המשתמש הנוכחי אהב אותו. דפים שעדיין אינם קיימים מחזירים `likeCount` של `0`.

העבר `includeUserInfo=public` כדי גם לקבל את המשתמשים שאהבו את העמוד, newest first. משתמשים עם פעילות פרופיל פרטית ומשתמשים אנונימיים מושמטים, וניתן לחפש עד 100 לייקים. רק `public` מתקבל כאן; `include-private` מחזיר `422` וזמין מ-[`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) עם מפתח API. בעמודים הדורשים SSO, `includeUserInfo` מחזיר `403` למשתמשים שלא יכולים לצפות בעמוד.

[inline-code-attrs-start title = 'דוגמת cURL של לייקים בעמוד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת לייקים בעמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת לייקים בעמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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