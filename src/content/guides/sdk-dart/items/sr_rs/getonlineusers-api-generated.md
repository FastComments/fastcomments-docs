Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ИД URL‑а странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Тијебрејкер курсора: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како називни спојци не би изгубили уносе. |

## Response

Returns: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | ИД URL‑а странице (очишћен на серверу).
final afterName = afterName_example; // String | Курсор: проследите nextAfterName из претходног одговора.
final afterUserId = afterUserId_example; // String | Тијебрејкер курсора: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како називни спојци не би изгубили уносе.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]