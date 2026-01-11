req
tenantId
afterId

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| afterId | string | query | Не |  |
| limit | integer | query | Не |  |
| tags | array | query | Не |  |
| sso | string | query | Не |  |
| isCrawler | boolean | query | Не |  |
| includeUserInfo | boolean | query | Не |  |

## Одговор

Враћа: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsPublic200Response.swift)

## Пример

[inline-code-attrs-start title = 'getFeedPostsPublic Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За било какав проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (опционо)
let limit = 987 // Int |  (опционо)
let tags = ["inner_example"] // [String] |  (опционо)
let sso = "sso_example" // String |  (опционо)
let isCrawler = true // Bool |  (опционо)
let includeUserInfo = true // Bool |  (опционо)

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