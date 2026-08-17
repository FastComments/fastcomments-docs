Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Текущие онлайн‑просмотрщики страницы: люди, чья сессия WebSocket в данный момент подписана на страницу.

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
Возвращает anonCount + totalCount (подписчики по всей комнате, включая анонимных зрителей, которых мы не перечисляем).

## Parameters

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищенный на сервере). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Тай-брейкер курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы привязки по имени не приводили к пропуску записей. |

## Response

Returns: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Идентификатор URL страницы (очищенный на сервере).
final afterName = afterName_example; // String | Курсор: передайте nextAfterName из предыдущего ответа.
final afterUserId = afterUserId_example; // String | Тай-брейкер курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы привязки по имени не приводили к пропуску записей.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]

---