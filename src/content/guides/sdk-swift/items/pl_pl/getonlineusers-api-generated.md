Obecnie online widzowie strony: osoby, których sesja websocket jest aktualnie subskrybowana na tę stronę.
Zwraca anonCount + totalCount (subskrybenci w całym pokoju, włącznie z anonimowymi widzami, których nie wyliczamy).

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak | Identyfikator adresu URL strony (oczyszczany po stronie serwera). |
| afterName | string | query | Nie | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | Nie | Rozstrzygacz kursorów: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów. |

## Odpowiedź

Zwraca: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemu zgłoś go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identyfikator adresu URL strony (oczyszczany po stronie serwera).
let afterName = "afterName_example" // String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. (opcjonalne)
let afterUserId = "afterUserId_example" // String | Rozstrzygacz kursorów: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów. (opcjonalne)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]