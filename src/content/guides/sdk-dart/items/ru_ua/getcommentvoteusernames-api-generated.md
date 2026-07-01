## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| dir | integer | query | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: `GetCommentVoteUserNamesSuccessResponse`

## Пример

[inline-code-attrs-start title = 'Пример getCommentVoteUserNames'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final dir = 56; // int | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCommentVoteUserNames(tenantId, commentId, dir, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentVoteUserNames: $e\n');
}
[inline-code-end]