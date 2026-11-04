Број лајкова за страницу, опционо са корисницима који су лајковали.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |
| includeUserInfo | string | query | Не | „public" изоставља кориснике са приватним профилима, „include-private" их укључује. |

## Одговор

Враћа: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageLikesAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getPageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још увек у бета верзији. За било какав проблем, пријавите преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" изоставља кориснике са приватним профилима, \"include-private\" их укључује. (optional)

DefaultAPI.getPageLikes(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]