## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | Yes |  |
| sso | string | query | No |  |

## Одговор

Враћа: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'lockComment Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // Стринг | 
final commentId = commentId_example; // Стринг | 
final broadcastId = broadcastId_example; // Стринг | 
final sso = sso_example; // Стринг | 

try {
    final result = api_instance.lockComment(tenantId, commentId, broadcastId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->lockComment: $e\n');
}
[inline-code-end]