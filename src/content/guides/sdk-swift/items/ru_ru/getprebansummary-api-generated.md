## Параметры

| Name | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| includeByUserIdAndEmail | boolean | query | Нет |  |
| includeByIP | boolean | query | Нет |  |
| includeByEmailDomain | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`PreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PreBanSummary.swift)

## Пример

[inline-code-attrs-start title = 'Пример getPreBanSummary'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все еще находятся в стадии бета. При возникновении проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeByUserIdAndEmail = true // Bool |  (необязательно)
let includeByIP = true // Bool |  (необязательно)
let includeByEmailDomain = true // Bool |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getPreBanSummary(commentId: commentId, includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso) { (response, error) in
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