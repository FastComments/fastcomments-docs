## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| includeByUserIdAndEmail | boolean | query | Не |  |
| includeByIP | boolean | query | Не |  |
| includeByEmailDomain | boolean | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`PreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PreBanSummary.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getPreBanSummary'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни кодове все още са бета. За всякакъв проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeByUserIdAndEmail = true // Bool |  (по избор)
let includeByIP = true // Bool |  (по избор)
let includeByEmailDomain = true // Bool |  (по избор)
let sso = "sso_example" // String |  (по избор)

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