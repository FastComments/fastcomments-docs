Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a “Members” section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищенный на сервере). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Разрешающий фактор курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда afterName установлен, чтобы при совпадении имён записи не терялись. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Идентификатор URL страницы (очищенный на сервере).
final afterName = afterName_example; // String | Курсор: передайте nextAfterName из предыдущего ответа.
final afterUserId = afterUserId_example; // String | Разрешающий фактор курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда afterName установлен, чтобы при совпадении имён записи не терялись.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]