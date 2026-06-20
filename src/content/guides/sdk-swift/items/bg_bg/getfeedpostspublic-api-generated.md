req
tenantId
afterId

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| afterId | string | query | Не |  |
| limit | integer | query | Не |  |
| tags | array | query | Не |  |
| sso | string | query | Не |  |
| isCrawler | boolean | query | Не |  |
| includeUserInfo | boolean | query | Не |  |

## Отговор

Връща: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getFeedPostsPublic Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери с код все още са в бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (незадължително)
let limit = 987 // Int |  (незадължително)
let tags = ["inner_example"] // [String] |  (незадължително)
let sso = "sso_example" // String |  (незадължително)
let isCrawler = true // Bool |  (незадължително)
let includeUserInfo = true // Bool |  (незадължително)

PublicAPI.getFeedPostsPublic(tenantId: tenantId, afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]