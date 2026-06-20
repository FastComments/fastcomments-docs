## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| spam | boolean | query | Не |  |
| permNotSpam | boolean | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за postSetCommentSpamStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери за код все още са в бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let spam = true // Bool |  (незадължително)
let permNotSpam = true // Bool |  (незадължително)
let sso = "sso_example" // String |  (незадължително)

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

---