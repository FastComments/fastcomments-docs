Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор на URL на страницата (почистено от сървъра). |
| afterName | string | query | No | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | No | Тайбрейкър за курсор: предайте nextAfterUserId от предишния отговор. Задължително, когато е зададен afterName, за да не се пропускат записи при равни имена. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодови откъси все още са в бета версия. За каквито и да е проблеми, моля, докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор на URL на страницата (почистено от сървъра).
let afterName = "afterName_example" // String | Курсор: предайте nextAfterName от предишния отговор. (по избор)
let afterUserId = "afterUserId_example" // String | Тайбрейкър за курсор: предайте nextAfterUserId от предишния отговор. Задължително, когато е зададен afterName, за да не се пропускат записи при равни имена. (по избор)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]