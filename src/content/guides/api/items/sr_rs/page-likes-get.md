[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Враћа број лајкова на страници и да ли је тренутни корисник лајковао страницу. Странице које још не постоје враћају `likeCount` од `0`.

Проследите `includeUserInfo=public` да бисте такође добили кориснике који су лајковали страницу, прво најновије. Корисници са приватном активностим профила и анонимни корисници се изостављају, а прегледа се до 100 лајкова. Овде се прихвата само `public`; `include-private` враћа `422` и доступно је из [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) уз API кључ. На страницама које захтевају SSO, `includeUserInfo` враћа `403` за кориснике који не могу да виде страницу.

[inline-code-attrs-start title = 'Пример cURL захтева за лајкове странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за лајкове странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI‑кодиран JSON вашег SSO објекта. Изоставите за анонимне кориснике. **/
    sso?: string
    /** Поставите на 'public' да укључите кориснике који су лајковали страницу. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за лајкове странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Укључено у случају грешке. **/
    reason?: string
    likeCount: number
    /** Да ли је корисник који шаље захтев лајковао страницу. **/
    didLike: boolean
    /** Број коментара највишег нивоа на страници. **/
    commentCount: number
    /** ИД који се користи за претплату на ажурирања уживо за ову страницу. **/
    urlIdWS: string
    /** Прво најновије. Поставља се само када је прослеђено includeUserInfo. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]