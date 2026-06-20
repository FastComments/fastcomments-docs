req
tenantId
afterId

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Не |  |
| limit | integer | query | Не |  |
| tags | array | query | Не |  |

## Одговор

Враћа: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getFeedPosts Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бети. За било какав проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (опционо)
let limit = 987 // Int |  (опционо)
let tags = ["inner_example"] // [String] |  (опционо)

DefaultAPI.getFeedPosts(tenantId: tenantId, afterId: afterId, limit: limit, tags: tags) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]