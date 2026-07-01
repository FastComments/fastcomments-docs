## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Ответ

Возвращает: `ModerationAPIChildCommentsResponse`

## Пример

[inline-code-attrs-start title = 'postCommentsByIds Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentsByIdsParams = CommentsByIdsParams(); // CommentsByIdsParams | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postCommentsByIds(tenantId, commentsByIdsParams, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postCommentsByIds: $e\n');
}
[inline-code-end]