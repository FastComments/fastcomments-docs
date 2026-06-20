## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| spam | boolean | query | Нет |  |
| permNotSpam | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример postSetCommentSpamStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бете. Если возникнет проблема, сообщите о ней по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let spam = true // Bool |  (необязательно)
let permNotSpam = true // Bool |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

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