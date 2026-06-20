---
Bulk gebruikersinformatie voor een tenant. Gegeven userIds, retourneert weergave-informatie van User / SSOUser.
Wordt gebruikt door de commentaar-widget om gebruikers te verrijken die net zijn verschenen via een presence event.
Geen pagina-context: privacy wordt uniform gehandhaafd (privéprofielen worden gemaskeerd).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Door komma's gescheiden userIds. |

## Antwoord

Retourneert: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getUsersInfo Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | Door komma's gescheiden userIds.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
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