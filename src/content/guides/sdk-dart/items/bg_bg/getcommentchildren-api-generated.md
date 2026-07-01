## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: `ModerationAPIChildCommentsResponse`

## Пример

[inline-code-attrs-start title = 'getCommentChildren Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCommentChildren(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getCommentChildren: $e\n');
}
[inline-code-end]

---