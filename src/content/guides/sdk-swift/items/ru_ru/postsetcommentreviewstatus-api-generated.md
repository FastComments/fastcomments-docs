## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| reviewed | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример postSetCommentReviewStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. При возникновении проблем, пожалуйста, сообщите по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let reviewed = true // Bool |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.postSetCommentReviewStatus(commentId: commentId, reviewed: reviewed, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]