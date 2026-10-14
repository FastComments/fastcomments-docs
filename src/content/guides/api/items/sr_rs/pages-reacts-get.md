[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Враћа број за сваку реакцију на страници, а по потреби и кориснике који су реаговали, као један унос по реакцији са њеним reaction id, најновије прво. Ово је серверска верзија [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` контролише који се корисници враћају:

- `public` изоставља кориснике који су поставили своју активност профила као приватну.
- `include-private` их укључује. Користите ово само када листа није приказана јавности.

Анонимни корисници се никада не укључују, а прегледа се до 100 реакција. Странице које не постоје не враћају `counts` и враћају празан `users` лист.

[inline-code-attrs-start title = 'Пример cURL захтева за реакције странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за реакције странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Изоставите да бисте добили само бројеве. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за реакције странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Укључено у случају грешке. **/
    reason?: string
    /** Број по ID-у реакције, на пример {"heart": 12, "laugh": 3}. Не поставља се када страница нема реакције. **/
    counts?: Record<string, number>
    /** Један унос по реакцији, најновије прво. Поставља се само када је прослеђен includeUserInfo. **/
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

#### Корисни савет

Запамтите да URI Encode вредности као што је `urlId`.