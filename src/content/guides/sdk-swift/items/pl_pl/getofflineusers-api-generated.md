Osoby komentujące wcześniej na stronie, które NIE są obecnie online. Sortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyrenderować sekcję "Members".
Paginacja kursorowa po commenterName: serwer przechodzi po częściowym indeksie {tenantId, urlId, commenterName}
od afterName w przód za pomocą $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak | Identyfikator adresu URL strony (oczyszczany po stronie serwera). |
| afterName | string | query | Nie | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | Nie | Kursor rozstrzygający remisy: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagany, gdy afterName jest ustawiony, aby remisy nazw nie gubiły wpisów. |

## Odpowiedź

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identyfikator adresu URL strony (oczyszczany po stronie serwera).
let afterName = "afterName_example" // String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. (optional)
let afterUserId = "afterUserId_example" // String | Kursor rozstrzygający remisy: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagany, gdy afterName jest ustawiony, aby remisy nazw nie gubiły wpisów. (optional)

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