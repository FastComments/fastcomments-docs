[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Додаје реакцију на страницу као тренутни корисник. Корисник може додати сваки id реакције само једном: поновно додавање успева са кодом `already-reacted` и не мења број. Корисник може додати неколико различитих реакција на исту страницу.

Ид‑ове реакција бирате ви и могу бити до 36 знакова. Страница се креира ако још не постоји. Проследите `title` да поставите или ажурирате наслов странице.

[inline-code-attrs-start title = 'Пример cURL захтева за реакцију странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за реакцију странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** Ид реакције, до 36 знакова. **/
    id: string
    /** Поставља наслов странице. **/
    title?: string
    /** URI‑кодиран JSON вашег SSO објекта. Изоставите за анонимне кориснике. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за реакцију странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' када је корисник већ додао ову реакцију. 'react-id-too-long' (HTTP 422) када је ид дужи од 36 знакова. У супротном се укључује у случају неуспеха. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]