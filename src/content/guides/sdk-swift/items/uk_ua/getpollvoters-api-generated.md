Хто проголосував за яку опцію. Анонімні опитування ніколи не відповідають, опитування лише для адміністраторів відповідають лише адміністратору або модератору.

## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| urlId | string | query | Ні |  |
| page | integer | query | Ні |  |
| search | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getPollVoters'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще у бета-версії. У разі проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String | (необов’язково)
let page = 987 // Int | (необов’язково)
let search = "search_example" // String | (необов’язково)
let sso = "sso_example" // String | (необов’язково)

PublicAPI.getPollVoters(tenantId: tenantId, commentId: commentId, options: PublicAPI.GetPollVotersOptions(urlId: urlId, page: page, search: search, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]