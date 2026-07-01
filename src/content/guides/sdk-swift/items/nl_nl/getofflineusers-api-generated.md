Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a “Members” section.  
Cursor pagination on commenterName: server walks the partial `{tenantId, urlId, commenterName}` index from afterName forward via `$gt`, no `$skip` cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Pagina-URL-identificatie (schoongemaakt server-side). |
| afterName | string | query | No | Cursor: geef nextAfterName door van de vorige response. |
| afterUserId | string | query | No | Cursor-onderbreker: geef nextAfterUserId door van de vorige response. Vereist wanneer afterName is ingesteld zodat naam-ties geen items laten vallen. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in beta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Pagina-URL-identificatie (schoongemaakt server-side).
let afterName = "afterName_example" // String | Cursor: geef nextAfterName door van de vorige response. (optional)
let afterUserId = "afterUserId_example" // String | Cursor-onderbreker: geef nextAfterUserId door van de vorige response. Vereist wanneer afterName is ingesteld zodat naam-ties geen items laten vallen. (optional)

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