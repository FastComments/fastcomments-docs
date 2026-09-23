---
[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Добавя реакция към страница от текущия потребител. Потребителят може да добави всяко ID на реакция само веднъж: повторното добавяне успява с кода `already-reacted` и не променя броя. Потребителят може да добави няколко различни реакции към една и съща страница.

ID‑тата на реакциите се избират от вас и могат да бъдат до 36 знака. Страницата се създава, ако все още не съществува. Предайте `title`, за да зададете или актуализирате заглавието на страницата.

[inline-code-attrs-start title = 'Пример cURL за реакция на страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за реакция на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** ID‑то на реакцията, до 36 знака. **/
    id: string
    /** Задава заглавието на страницата. **/
    title?: string
    /** URI‑кодиран JSON на вашия SSO обект. Пропуснете за анонимни потребители. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за реакция на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' когато потребителят вече е добавил тази реакция. 'react-id-too-long' (HTTP 422) когато ID‑то е над 36 знака. В противен случай се включва при неуспех. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Включено при неуспех. **/
    reason?: string
}
[inline-code-end]

---