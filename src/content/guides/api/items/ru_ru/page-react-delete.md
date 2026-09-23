[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Удаляет одну из реакций текущего пользователя со страницы. Если пользователь не добавлял эту реакцию, запрос считается успешным с кодом `no-react` и количество реакций не изменяется.

[inline-code-attrs-start title = 'Пример cURL для удаления реакции страницы'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса удаления реакции страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа удаления реакции страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' when the user had not added this reaction. Otherwise included on failure. **/
    code?: 'no-react' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]