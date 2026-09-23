[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Връща броя за всяка реакция на страница и кои реакции е добавил текущият потребител.

[inline-code-attrs-start title = 'Пример cURL за реакциите на страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за реакциите на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI кодиран JSON на вашия SSO обект. Пропуснете за анонимни потребители. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за реакциите на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: string
    /** Включено при неуспех. **/
    reason?: string
    /** Брой за всяко ID на реакция, например {"heart": 12, "laugh": 3}. Не се задава, когато страницата няма реакции. **/
    counts?: Record<string, number>
    /** ID‑тата на реакциите, които потребителят, правещ заявката, е добавил. Не се задава, ако не е добавил нито една. **/
    reactedIds?: string[]
}
[inline-code-end]