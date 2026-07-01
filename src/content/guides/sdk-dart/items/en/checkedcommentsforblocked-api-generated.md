## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentIds | string | query | Yes | A comma separated list of comment ids. |
| sso | string | query | No |  |

## Response

Returns: `CheckBlockedCommentsResponse`

## Example

[inline-code-attrs-start title = 'checkedCommentsForBlocked Example'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | A comma separated list of comment ids.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]
