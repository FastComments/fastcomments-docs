Past commenters on the page who are NOT currently online. Sorted by displayName.  
Използвайте това след като изчерпите /users/online, за да покажете секция „Members“.  
Cursor pagination on commenterName: server walks the partial `{tenantId, urlId, commenterName}` index from afterName forward via $gt, no $skip cost.  
Курсорна пагинация по commenterName: сървърът обхожда частичния `{tenantId, urlId, commenterName}` индекс от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор за URL на страницата (почистен от сървъра). |
| afterName | string | query | No | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | No | Курсор за разрешаване на равенства: предайте nextAfterUserId от предишния отговор. Задължително, когато е зададен afterName, за да равенствата по име не премахват записите. |

## Отговор

Връща: `PageUsersOfflineResponse`

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final urlId = urlId_example; // String | Идентификатор за URL на страницата (почистен от сървъра).
final afterName = afterName_example; // String | Курсор: предайте nextAfterName от предишния отговор.
final afterUserId = afterUserId_example; // String | Курсор за разрешаване на равенства: предайте nextAfterUserId от предишния отговор. Задължително, когато е зададен afterName, за да равенствата по име не премахват записите.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]