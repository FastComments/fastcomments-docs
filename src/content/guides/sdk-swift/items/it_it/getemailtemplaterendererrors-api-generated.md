## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplateRenderErrorsResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getEmailTemplateRenderErrors'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in fase beta. Per qualsiasi problema, si prega di segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let skip = 987 // Double |  (opzionale)

DefaultAPI.getEmailTemplateRenderErrors(tenantId: tenantId, id: id, skip: skip) { (response, error) in
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