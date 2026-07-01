Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

Obecnie oglądający stronę: osoby, których sesja WebSocket jest aktualnie subskrybowana do tej strony.  
Zwraca anonCount + totalCount (subskrybenci w całym pokoju, w tym anonimowi oglądający, których nie wymieniamy).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (czyszczony po stronie serwera). |
| afterName | string | query | No | Kursor: przekaż nextAfterName z poprzedzi odpowiedzi. |
| afterUserId | string | query | No | Rozwiązanie remisu kursora: przekaż nextAfterUserId z poprzedzi odpowiedzi. Wymagane, gdy afterName jest ustawiony, aby w przypadku remisów nazw nie pominięto wpisów. |

## Response

Returns: `PageUsersOnlineResponse`

Zwraca: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Przykład getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identyfikator URL strony (czyszczony po stronie serwera).
final afterName = afterName_example; // String | Kursor: przekaż nextAfterName z poprzedzi odpowiedzi.
final afterUserId = afterUserId_example; // String | Rozwiązanie remisu kursora: przekaż nextAfterUserId z poprzedzi odpowiedzi. Wymagane, gdy afterName jest ustawiony, aby w przypadku remisów nazw nie pominięto wpisów.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]