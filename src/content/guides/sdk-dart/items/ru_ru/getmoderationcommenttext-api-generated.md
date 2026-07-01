## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|--------------|-------------|----------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: `GetCommentTextResponse`

## Пример

[inline-code-attrs-start title = 'Пример getModerationCommentText'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getModerationCommentText(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getModerationCommentText: $e\n');
}
[inline-code-end]