Unsubscribes (REST hook unsubscribe). Only subscriptions created through this API can be deleted here; dashboard-managed webhooks are edited in the dashboard.

Anulowania subskrypcji (REST hook unsubscribe). Tylko subskrypcje utworzone za pośrednictwem tego API mogą być tutaj usunięte; webhooki zarządzane w panelu są edytowane w panelu.

## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Example

[inline-code-attrs-start title = 'Przykład deleteWebhook'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie jakichkolwiek problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.deleteWebhook(tenantId: tenantId, id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]