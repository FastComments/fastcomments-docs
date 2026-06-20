## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| postIds | array | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FeedPostsStatsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getFeedPostsStats'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са бета. За всеки проблем, моля докладвайте на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postIds = ["inner_example"] // [String] | 
let sso = "sso_example" // String |  (незадължително)

PublicAPI.getFeedPostsStats(tenantId: tenantId, postIds: postIds, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]