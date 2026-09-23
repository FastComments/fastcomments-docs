[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Харесва страница като текущия потребител. Всеки потребител може да харесва страница само веднъж: повторно харесване успява с кода `already-liked` и не променя броя.

Страницата се създава, ако все още не съществува. Предайте `title`, за да зададете или актуализирате заглавието на страницата.

[inline-code-attrs-start title = 'Пример за cURL заявка за харесване на страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за харесване на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Задава заглавието на страницата. **/
    title?: string
    /** URI кодиран JSON на вашия SSO обект. Пропуснете за анонимни потребители. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за харесване на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' когато потребителят вече е харесал страницата. В противен случай се включва при неуспех. **/
    code?: 'already-liked' | string
    /** Включено при неуспех. **/
    reason?: string
}
[inline-code-end]