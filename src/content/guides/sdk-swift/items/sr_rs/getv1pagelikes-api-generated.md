## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| sso | string | query | Не |  |
| includeUserInfo | string | query | Не | Када је постављено, враћа кориснике који су лајковали страницу (аватаре и корисничка имена). Прихваћено је само "public", што изоставља приватне профиле. |

## Одговор

Враћа: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Пример

[inline-code-attrs-start title = 'Primer getV1PageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, молимо пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (опционално)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Када је постављено, враћа кориснике који су лајковали страницу (аватаре и корисничка имена). Прихваћено је само \"public\", што изоставља приватне профиле. (опционално)

PublicAPI.getV1PageLikes(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV1PageLikesOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]