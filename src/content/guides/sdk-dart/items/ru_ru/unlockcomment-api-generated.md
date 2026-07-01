## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | путь | Да |  |
| commentId | string | путь | Да |  |
| broadcastId | string | запрос | Да |  |
| sso | string | запрос | Нет |  |

## Ответ

Возвращает: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'unLockComment Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.unLockComment(tenantId, commentId, broadcastId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->unLockComment: $e\n');
}
[inline-code-end]