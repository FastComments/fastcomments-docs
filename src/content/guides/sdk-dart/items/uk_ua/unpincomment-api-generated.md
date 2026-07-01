## Параметри

| Ім'я | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | Yes |  |
| sso | string | query | No |  |

## Відповідь

Повертає: `ChangeCommentPinStatusResponse`

## Приклад

[inline-code-attrs-start title = 'unPinComment Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.unPinComment(tenantId, commentId, broadcastId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->unPinComment: $e\n');
}
[inline-code-end]