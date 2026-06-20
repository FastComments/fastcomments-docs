## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| spam | boolean | query | No |  |
| permNotSpam | boolean | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад postSetCommentSpamStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще перебувають у бета-версії. Якщо знайдете проблему, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let spam = true // Bool |  (необов'язковий)
let permNotSpam = true // Bool |  (необов'язковий)
let sso = "sso_example" // String |  (необов'язковий)

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