Osoby, które komentowały stronę w przeszłości, a które NIE są obecnie online. Sortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyrenderować sekcję "Members" (Członkowie).
Paginacja kursorowa po commenterName: serwer przechodzi po częściowym indeksie {tenantId, urlId, commenterName}
od afterName w przód za pomocą $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak | Identyfikator URL strony (czyszczony po stronie serwera). |
| afterName | string | query | Nie | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | Nie | Kursor rozstrzygający remisy: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagany, gdy ustawiono afterName, aby remisy nazw nie powodowały utraty wpisów. |

## Odpowiedź

Zwraca: `PageUsersOfflineResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identyfikator URL strony (czyszczony po stronie serwera).
final afterName = afterName_example; // String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi.
final afterUserId = afterUserId_example; // String | Kursor rozstrzygający remisy: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagany, gdy ustawiono afterName, aby remisy nazw nie powodowały utraty wpisów.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]