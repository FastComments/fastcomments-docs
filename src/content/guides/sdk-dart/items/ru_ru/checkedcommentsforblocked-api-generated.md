## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentIds | string | query | Да | Список идентификаторов комментариев, разделенных запятыми. |
| sso | string | query | Нет |  |

## Ответ

Возвращает: `CheckBlockedCommentsResponse`

## Пример

[inline-code-attrs-start title = 'checkedCommentsForBlocked Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Список идентификаторов комментариев, разделенных запятыми.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]