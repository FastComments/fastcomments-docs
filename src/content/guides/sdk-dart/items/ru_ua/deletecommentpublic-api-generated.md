## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Да |  |
| editKey | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: `PublicAPIDeleteCommentResponse`

## Пример

[inline-code-attrs-start title = 'deleteCommentPublic Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteCommentPublic(tenantId, commentId, broadcastId, DeleteCommentPublicOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Исключение при вызове PublicApi->deleteCommentPublic: $e\n');
}
[inline-code-end]