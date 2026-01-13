## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | zapytanie | Tak |  |

## Response

Zwraca: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateModerator200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład createModerator'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są wciąż w fazie beta. W przypadku problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createModeratorBody = CreateModeratorBody(name: "name_example", email: "email_example", userId: "userId_example", moderationGroupIds: ["moderationGroupIds_example"]) // CreateModeratorBody | 

DefaultAPI.createModerator(tenantId: tenantId, createModeratorBody: createModeratorBody) { (response, error) in
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