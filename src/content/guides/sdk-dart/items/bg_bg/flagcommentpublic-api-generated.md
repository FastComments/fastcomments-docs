## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| isFlagged | boolean | query | Да |  |
| sso | string | query | Не |  |

## Response

Returns: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'flagCommentPublic Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
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