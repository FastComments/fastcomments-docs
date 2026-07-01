Past commenters on the page who are NOT currently online. Sorted by displayName.  
Użyj tego po wyczerpaniu /users/online, aby wyświetlić sekcję „Members”.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  

## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (wyczyszczony po stronie serwera). |
| afterName | string | query | No | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Rozdzielacz tie w kursorze: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy ustawiono afterName, aby powiązania nazw nie pomijały wpisów. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
// Ten kod jest nadal w wersji beta. W razie problemów prosimy zgłaszać je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identyfikator URL strony (wyczyszczony po stronie serwera).
let afterName = "afterName_example" // String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. (opcjonalnie)
let afterUserId = "afterUserId_example" // String | Rozdzielacz tie w kursorze: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy ustawiono afterName, aby powiązania nazw nie pomijały wpisów. (opcjonalnie)

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