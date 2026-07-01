Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Ідентифікатор URL сторінки (очищений на сервері). |
| afterName | string | query | No | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | No | Тай-брейкер курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли afterName встановлено, щоб уникнути втрати записів через однакові імена. |

## Відповідь

Повертає: `PageUsersOfflineResponse`

## Приклад

[inline-code-attrs-start title = 'getOfflineUsers Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Ідентифікатор URL сторінки (очищений на сервері).
final afterName = afterName_example; // String | Курсор: передайте nextAfterName з попередньої відповіді.
final afterUserId = afterUserId_example; // String | Тай-брейкер курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли afterName встановлено, щоб уникнути втрати записів через однакові імена.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]