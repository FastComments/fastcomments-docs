## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBannedUsersFromCommentResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getBanUsersFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (опционо)

ModerationAPI.getBanUsersFromComment(commentId: commentId, sso: sso) { (response, error) in
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