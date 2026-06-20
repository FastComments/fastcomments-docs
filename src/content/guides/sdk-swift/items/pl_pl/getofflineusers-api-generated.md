---
Byli komentujący na stronie, którzy NIE są obecnie online. Posortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyrenderować sekcję "Członkowie".
Paginacja kursorowa po commenterName: serwer przeszukuje częściowy indeks {tenantId, urlId, commenterName} od afterName w przód przy użyciu $gt, bez kosztu $skip.

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | ścieżka | Tak |  |
| urlId | string | zapytanie | Tak | Identyfikator URL strony (oczyszczany po stronie serwera). |
| afterName | string | zapytanie | Nie | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | zapytanie | Nie | Kursor — rozstrzygacz remisu: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów. |

## Odpowiedź

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemu zgłoś go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identyfikator URL strony (oczyszczany po stronie serwera).
let afterName = "afterName_example" // String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. (opcjonalne)
let afterUserId = "afterUserId_example" // String | Rozstrzygacz remisu kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów. (opcjonalne)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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