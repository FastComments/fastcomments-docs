## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateModerator'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in fase beta. Per qualsiasi problema, segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateModeratorBody = UpdateModeratorBody(name: "name_example", email: "email_example", userId: "userId_example", moderationGroupIds: ["moderationGroupIds_example"]) // UpdateModeratorBody | 

DefaultAPI.updateModerator(tenantId: tenantId, id: id, updateModeratorBody: updateModeratorBody) { (response, error) in
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