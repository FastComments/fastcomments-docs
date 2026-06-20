---
Zbiorcze informacje o użytkownikach dla najemcy. Dla podanych userIds zwracane są informacje wyświetlane z User / SSOUser.
Używane przez widget komentarzy do wzbogacenia informacji o użytkownikach, którzy właśnie pojawili się poprzez zdarzenie obecności.
Brak kontekstu strony: prywatność jest egzekwowana jednolicie (profile prywatne są maskowane).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | userIds rozdzielone przecinkami. |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## Example

[inline-code-attrs-start title = 'Przykład getUsersInfo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemów zgłoś je pod http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | userIds rozdzielone przecinkami.

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