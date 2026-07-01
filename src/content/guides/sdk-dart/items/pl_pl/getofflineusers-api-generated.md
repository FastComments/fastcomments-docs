Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (oczyszczony po stronie serwera). |
| afterName | string | query | No | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby powiązania nazw nie powodowały pomijania wpisów. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identyfikator URL strony (oczyszczony po stronie serwera).
final afterName = afterName_example; // String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi.
final afterUserId = afterUserId_example; // String | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby powiązania nazw nie powodowały pomijania wpisów.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]