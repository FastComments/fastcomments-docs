## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| fromName | string | query | Yes |  |

## Response

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Example

[inline-code-attrs-start title = 'Beispiel für sendInvite'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let fromName = "fromName_example" // String | 

DefaultAPI.sendInvite(tenantId: tenantId, id: id, fromName: fromName) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]