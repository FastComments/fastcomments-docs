Like count for a page, optionally with the users who liked it.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| includeUserInfo | string | query | No | "public" изоставља кориснике са приватним профилима, "include-private" их укључује. |

## Одговор

Returns: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageLikesAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'Primer getPageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, молимо пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" изоставља кориснике са приватним профилима, \"include-private\" их укључује. (опционално)

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

---