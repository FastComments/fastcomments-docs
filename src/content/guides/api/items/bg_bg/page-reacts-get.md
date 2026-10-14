[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Връща броя за всяка реакция на страница и кои реакции е добавил текущият потребител.

Преминете `includeUserInfo=public`, за да получите и потребителите, които са реагирали, като един запис за всяка реакция с нейния идентификатор, най‑новите първи. Потребителите с частна активност в профила и анонимните потребители се изключват, а се проверяват до 100 реакции. Тук се приема само `public`; `include-private` връща `422` и е достъпен от [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) с API ключ. На страници, изискващи SSO, `includeUserInfo` връща `403` за потребители, които не могат да видят страницата.

[inline-code-attrs-start title = 'Пример cURL за Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who reacted. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    /** Count per reaction id, for example {"heart": 12, "laugh": 3}. Not set when the page has no reactions. **/
    counts?: Record<string, number>
    /** The reaction ids the user making the request has added. Not set when they have added none. **/
    reactedIds?: string[]
    /** One entry per react, most recent first. Only set when includeUserInfo is passed. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** The reaction id, for example "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]