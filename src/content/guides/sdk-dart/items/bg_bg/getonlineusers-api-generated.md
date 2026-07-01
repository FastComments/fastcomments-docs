---
Текущо онлайн зрители на страница: хора, чиито уебсокет сесии са абонирани за страницата в момента.  
Връща anonCount + totalCount (абонати в цялата стая, включително анонимни зрители, които не изброяваме).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор на URL на страницата (почистен от сървъра). |
| afterName | string | query | No | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | No | Тайбрейк на курсора: предайте nextAfterUserId от предишния отговор. Задължително, когато afterName е зададен, за да не се изпускат записи при равни имена. |

## Response

Returns: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Пример за getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Идентификатор на URL на страницата (почистен от сървъра).
final afterName = afterName_example; // String | Курсор: предайте nextAfterName от предишния отговор.
final afterUserId = afterUserId_example; // String | Тайбрейк на курсора: предайте nextAfterUserId от предишния отговор. Задължително, когато afterName е зададен, за да не се изпускат записи при равни имена.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]

---