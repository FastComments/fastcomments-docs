Коментатори на сторінці, які наразі НЕ в мережі. Відсортовані за displayName.
Використовуйте це після вичерпання /users/online для відображення розділу "Учасники".
Курсорна пагінація за commenterName: сервер проходить частковим індексом {tenantId, urlId, commenterName}
від afterName вперед через $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Ідентифікатор URL сторінки (очищається на сервері). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Додатковий критерій курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язковий, коли afterName встановлено, щоб записи з однаковими іменами не втрачалися. |

## Відповідь

Повертає: `PageUsersOfflineResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Ідентифікатор URL сторінки (очищається на сервері).
final afterName = afterName_example; // String | Курсор: передайте nextAfterName з попередньої відповіді.
final afterUserId = afterUserId_example; // String | Додатковий критерій курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язковий, коли afterName встановлено, щоб записи з однаковими іменами не втрачалися.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]