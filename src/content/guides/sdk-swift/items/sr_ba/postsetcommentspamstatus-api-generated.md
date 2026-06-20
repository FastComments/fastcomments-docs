## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| spam | boolean | query | Не |  |
| permNotSpam | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Примјер

[inline-code-attrs-start title = 'postSetCommentSpamStatus Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Ови примјери кода су и даље у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let spam = true // Bool |  (необавезно)
let permNotSpam = true // Bool |  (необавезно)
let sso = "sso_example" // String |  (необавезно)

ModerationAPI.postSetCommentSpamStatus(commentId: commentId, spam: spam, permNotSpam: permNotSpam, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]