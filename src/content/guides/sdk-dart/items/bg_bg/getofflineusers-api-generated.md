Минали коментатори на страницата, които в момента НЕ са онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да визуализирате секция "Членове".
Пагинация с курсор по commenterName: сървърът обхожда частичния индекс {tenantId, urlId, commenterName}
от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (изчиства се от сървъра). |
| afterName | string | query | Не | Курсор: подайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Допълнителен критерий за курсора: подайте nextAfterUserId от предишния отговор. Задължителен, когато afterName е зададен, така че съвпаденията по име да не пропуснат записи. |

## Отговор

Връща: `PageUsersOfflineResponse`

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Идентификатор на URL на страницата (изчиства се от сървъра).
final afterName = afterName_example; // String | Курсор: подайте nextAfterName от предишния отговор.
final afterUserId = afterUserId_example; // String | Допълнителен критерий за курсора: подайте nextAfterUserId от предишния отговор. Задължителен, когато afterName е зададен, така че съвпаденията по име да не пропуснат записи.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]