## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |

## Response

Returns: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'putCloseThread Example'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putCloseThread(tenantId, urlId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putCloseThread: $e\n');
}
[inline-code-end]
