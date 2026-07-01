Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Ідентифікатор URL сторінки (очищений на сервері). |
| afterName | string | query | No | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | No | Тай-брейкер курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли afterName встановлено, щоб уникнути пропуску записів через однакові імена. |

## Response

Returns: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'getOnlineUsers Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Ідентифікатор URL сторінки (очищений на сервері).
final afterName = afterName_example; // String | Курсор: передайте nextAfterName з попередньої відповіді.
final afterUserId = afterUserId_example; // String | Тай-брейкер курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли afterName встановлено, щоб уникнути пропуску записів через однакові імена.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]