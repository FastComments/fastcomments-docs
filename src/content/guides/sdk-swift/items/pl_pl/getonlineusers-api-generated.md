Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (czyszczony po stronie serwera). |
| afterName | string | query | No | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Rozstrzygacz dla kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy ustawiono afterName, aby w sytuacji remisów nazw nie pomijać wpisów. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Przykład getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w fazie beta. W razie jakichkolwiek problemów prosimy zgłaszać je pod http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identyfikator URL strony (czyszczony po stronie serwera).
let afterName = "afterName_example" // String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. (opcjonalny)
let afterUserId = "afterUserId_example" // String | Rozstrzygacz dla kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy ustawiono afterName, aby w sytuacji remisów nazw nie pomijać wpisów. (opcjonalny)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]