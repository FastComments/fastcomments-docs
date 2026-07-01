## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |
| isFlagged | boolean | query | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `APIEmptyResponse`

## Приклад

[inline-code-attrs-start title = 'flagCommentPublic Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final isFlagged = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.flagCommentPublic(tenantId, commentId, isFlagged, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->flagCommentPublic: $e\n');
}
[inline-code-end]