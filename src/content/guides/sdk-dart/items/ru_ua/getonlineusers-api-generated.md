Текущие онлайн‑просмотрщики страницы: пользователи, чья WebSocket‑сессия подписана на страницу в данный момент.  
Возвращает anonCount + totalCount (подписчики в комнате, включая анонимных зрителей, которые не перечисляются).

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищенный на стороне сервера). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Разрешитель конфликтов курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы привязки по имени не пропускали записи. |

## Ответ

Returns: `PageUsersOnlineResponse`

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Идентификатор URL страницы (очищенный на стороне сервера).
final afterName = afterName_example; // String | Курсор: передайте nextAfterName из предыдущего ответа.
final afterUserId = afterUserId_example; // String | Разрешитель конфликтов курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы привязки по имени не пропускали записи.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]