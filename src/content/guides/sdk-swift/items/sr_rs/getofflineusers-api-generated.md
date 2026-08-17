Past commenters on the page who are NOT currently online. Sorted by displayName.
Use this after exhausting /users/online to render a "Members" section.
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Тијебрејкер за курсор: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како би се у случају везаних имена (name-ties) не изгубили уноси. |

## Response

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, молимо пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Идентификатор URL странице (очишћен на серверу).
let afterName = "afterName_example" // String | Курсор: проследите nextAfterName из претходног одговора. (optional)
let afterUserId = "afterUserId_example" // String | Тијебрејкер за курсор: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како би се у случају везаних имена (name-ties) не изгубили уноси. (optional)

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