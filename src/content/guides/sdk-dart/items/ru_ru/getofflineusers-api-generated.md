Прошлые комментаторы на странице, которые НЕ находятся сейчас в сети. Отсортированы по displayName.
Используйте этот метод после исчерпания /users/online для отображения раздела «Участники».
Курсорная пагинация по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName}
от afterName вперёд через $gt, без затрат на $skip.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на сервере). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Тай-брейкер курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы совпадающие имена не приводили к потере записей. |

## Ответ

Возвращает: `PageUsersOfflineResponse`

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Идентификатор URL страницы (очищается на сервере).
final afterName = afterName_example; // String | Курсор: передайте nextAfterName из предыдущего ответа.
final afterUserId = afterUserId_example; // String | Тай-брейкер курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы совпадающие имена не приводили к потере записей.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]