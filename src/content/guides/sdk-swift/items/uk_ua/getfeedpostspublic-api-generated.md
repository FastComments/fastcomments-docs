req
tenantId
afterId

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| afterId | string | query | Ні |  |
| limit | integer | query | Ні |  |
| tags | array | query | Ні |  |
| sso | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |
| includeUserInfo | boolean | query | Ні |  |

## Відповідь

Повертає: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getFeedPostsPublic Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду ще є бета-версією. У випадку будь-яких проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (необов'язковий)
let limit = 987 // Int |  (необов'язковий)
let tags = ["inner_example"] // [String] |  (необов'язковий)
let sso = "sso_example" // String |  (необов'язковий)
let isCrawler = true // Bool |  (необов'язковий)
let includeUserInfo = true // Bool |  (необов'язковий)

PublicAPI.getFeedPostsPublic(tenantId: tenantId, options: PublicAPI.GetFeedPostsPublicOptions(afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---