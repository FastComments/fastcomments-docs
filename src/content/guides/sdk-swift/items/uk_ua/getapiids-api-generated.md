## Параметри

| Назва | Тип | Розташування | Обовʼязково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| afterId | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getApiIds Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще в бета-версії. При будь‑якій проблемі, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let textSearch = "textSearch_example" // String |  (необов’язковий)
let byIPFromComment = "byIPFromComment_example" // String |  (необов’язковий)
let filters = "filters_example" // String |  (необов’язковий)
let searchFilters = "searchFilters_example" // String |  (необов’язковий)
let afterId = "afterId_example" // String |  (необов’язковий)
let demo = true // Bool |  (необов’язковий)
let sso = "sso_example" // String |  (необов’язковий)

ModerationAPI.getApiIds(tenantId: tenantId, options: ModerationAPI.GetApiIdsOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, afterId: afterId, demo: demo, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]