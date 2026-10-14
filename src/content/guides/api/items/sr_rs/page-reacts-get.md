[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Враћа број за сваку реакцију на страници, и које реакције је тренутни корисник додао.

Проследите `includeUserInfo=public` да бисте такође добили кориснике који су реаговали, као један унос по реакцији са њеним ID-ом реакције, прво најновије. Корисници са приватном активностим профила и анонимни корисници се изостављају, а прегледа се до 100 реакција. Овде се прихвата само `public`; `include-private` враћа `422` и доступно је из [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) уз API кључ. На страницама које захтевају SSO, `includeUserInfo` враћа `403` за кориснике који не могу да виде страницу.

[inline-code-attrs-start title = 'Пример cURL захтева за Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI‑кодирани JSON вашег SSO објекта. Изоставите за анонимне кориснике. **/
    sso?: string
    /** Подесите на 'public' да укључите кориснике који су реаговали. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Укључено у случају грешке. **/
    reason?: string
    /** Број по ID-у реакције, на пример {"heart": 12, "laugh": 3}. Није постављено када страница нема реакције. **/
    counts?: Record<string, number>
    /** ID‑еви реакција које је корисник послао додао. Није постављено када их није додао. **/
    reactedIds?: string[]
    /** Један унос по реакцији, прво најновије. Поставља се само када је прослеђен includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** ID реакције, на пример "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]